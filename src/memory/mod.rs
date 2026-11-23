//! Long-term memory: durable facts and preferences kept in the Crustly
//! database and recalled into later requests (docs/improve-memory.md).
//!
//! [`MemoryStore`] is the public API. Every write goes through
//! [`MemoryStore::save`], which applies the rules of section 8. Every request
//! reads through [`MemoryStore::recall`], which applies section 9. Errors
//! inside the store are returned as `Result`; callers on the request path log
//! them and carry on, so memory never fails a request.

pub mod embed;
pub mod model;
pub mod project;
pub mod recall;
pub mod secret_scan;
pub mod transfer;
pub mod vector;

use crate::config::MemoryConfig;
use crate::db::repository::memory::InsertMemory;
use crate::db::repository::MemoryRepository;
use anyhow::Result;
use chrono::Utc;
use model::{
    content_hash, normalize, validate, Memory, MemoryCategory, MemoryScope, MemorySource,
    MemoryStatus, NewMemory, ProjectId, RejectReason, SaveOutcome,
};
use project::ProjectIdentity;
use recall::Ranked;
use sqlx::SqlitePool;
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

/// Importance used when a save gives none, or an out-of-range one.
const DEFAULT_IMPORTANCE: u8 = 3;

/// Recent-work lines recalled on a session's first request, and their cap.
const RECENT_WORK_ENTRIES: usize = 3;
const RECENT_WORK_MAX_TOKENS: u32 = 150;

/// Upper bound on keyword matches read per request.
const KEYWORD_LIMIT: i64 = 1_000;

/// What recall returned for one request, for tests and diagnostics.
#[derive(Debug, Clone, PartialEq)]
pub struct RecallRequest<'a> {
    pub project: &'a ProjectIdentity,
    pub message: &'a str,
    /// The message before this one, used for short follow-up requests.
    pub previous: Option<&'a str>,
    /// Context window of the model that will receive the block, in tokens.
    pub window: u32,
    /// True for a session's first request, which also gets recent work.
    pub first_request: bool,
    /// Project root, where the progress log is read from.
    pub root: &'a Path,
}

/// Which memories a search covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchScope {
    All,
    Global,
    Project,
}

/// Public entry point for long-term memory.
pub struct MemoryStore {
    repo: MemoryRepository,
    config: MemoryConfig,
    /// Notices about saves, drained by [`MemoryStore::take_notices`].
    notices: Mutex<Vec<String>>,
}

impl MemoryStore {
    pub fn new(pool: SqlitePool, config: MemoryConfig) -> Self {
        Self {
            repo: MemoryRepository::new(pool),
            config: config.sanitized(),
            notices: Mutex::new(Vec::new()),
        }
    }

    /// Record the project before its memories are used.
    pub async fn register_project(&self, project: &ProjectIdentity) -> Result<()> {
        self.repo.upsert_project(project, Utc::now()).await
    }

    /// Save a memory, applying section 8: normalise, validate, scan for
    /// secrets, reinforce or touch an exact duplicate, check any replacement,
    /// then insert. Rejections are returned, not raised, so the caller can
    /// show the reason to the user or the model.
    pub async fn save(&self, new: NewMemory) -> Result<SaveOutcome> {
        let now = Utc::now();
        if let Err(reason) = validate(&new.content) {
            return Ok(SaveOutcome::Rejected(reason));
        }
        let content = normalize(&new.content);
        if let Some(rule) = secret_scan::scan(&content) {
            return Ok(SaveOutcome::Rejected(RejectReason::Secret { rule }));
        }

        let actor = actor_for(new.source);
        let hash = content_hash(&content);
        if let Some(existing) = self.repo.find_by_hash(&new.scope, &hash).await? {
            if new.source == MemorySource::User {
                self.repo.reinforce(existing.id, actor, now).await?;
            } else {
                // A repeat by the model is not evidence of importance (§8 step 4).
                self.repo.touch(existing.id, now).await?;
            }
            return Ok(SaveOutcome::Duplicate(existing.id));
        }

        let replaced = match new.replaces {
            Some(old_id) => match self.repo.get(old_id).await? {
                Some(old) if old.status != MemoryStatus::Active => {
                    return Ok(SaveOutcome::Rejected(RejectReason::NotFound))
                }
                Some(old) if old.scope != new.scope => {
                    return Ok(SaveOutcome::Rejected(RejectReason::ScopeMismatch))
                }
                Some(old)
                    if old.source == MemorySource::User && new.source != MemorySource::User =>
                {
                    return Ok(SaveOutcome::Rejected(RejectReason::SavedByUser))
                }
                Some(old) => Some(old),
                None => return Ok(SaveOutcome::Rejected(RejectReason::NotFound)),
            },
            None => None,
        };

        let id = Uuid::new_v4();
        let importance = if (1..=5).contains(&new.importance) {
            new.importance
        } else {
            DEFAULT_IMPORTANCE
        };
        self.repo
            .insert(
                &InsertMemory {
                    id,
                    scope: new.scope.clone(),
                    category: new.category,
                    content: content.clone(),
                    content_hash: hash,
                    source: new.source,
                    importance,
                    status: MemoryStatus::Active,
                    session_id: new.session_id,
                    expires_at: new.expires_at,
                    now,
                },
                actor,
            )
            .await?;
        if let Some(old) = replaced {
            self.repo.supersede(old.id, id, actor, now).await?;
        }
        self.notify(&new.scope, new.category.as_str(), id, &content);
        Ok(SaveOutcome::Created(id))
    }

    /// The memory block for one request, or `None` when nothing is recalled.
    /// Memory failures are logged and return `None`: they never fail a request.
    pub async fn recall(&self, request: &RecallRequest<'_>) -> Option<String> {
        match self.try_recall(request).await {
            Ok(block) => block,
            Err(err) => {
                tracing::warn!("memory recall skipped: {err:#}");
                None
            }
        }
    }

    async fn try_recall(&self, request: &RecallRequest<'_>) -> Result<Option<String>> {
        let now = Utc::now();
        let project_id: ProjectId = request.project.id.clone();
        let budget = self.config.recall_max_tokens.min(request.window / 16);

        let candidates = self.repo.candidates(&project_id, now).await?;
        if candidates.is_empty() {
            return Ok(None);
        }

        // Keyword scores, when the message has searchable words.
        let keyword_hits: Vec<(Memory, f64)> =
            match recall::fts_match_expr(request.message, request.previous) {
                Some(expr) => {
                    self.repo
                        .fts_search(&expr, &project_id, now, KEYWORD_LIMIT)
                        .await?
                }
                None => Vec::new(),
            };
        let ranked: Vec<Ranked> = candidates
            .into_iter()
            .map(|memory| {
                let keyword = keyword_hits
                    .iter()
                    .find(|(hit, _)| hit.id == memory.id)
                    .map(|(_, bm25)| *bm25);
                Ranked { memory, keyword }
            })
            .collect();

        let selected = recall::select(&ranked, budget, self.config.pinned_min_importance, now);
        if !selected.is_empty() {
            let ids: Vec<Uuid> = selected.iter().map(|m| m.id).collect();
            if let Err(err) = self.repo.record_recall(&ids, now).await {
                tracing::warn!("could not record recall: {err:#}");
            }
        }

        let recent = if request.first_request && self.config.recent_work {
            crate::llm::agent::project_memory::recent_work(
                request.root,
                RECENT_WORK_ENTRIES,
                RECENT_WORK_MAX_TOKENS.min(budget),
            )
        } else {
            Vec::new()
        };
        Ok(recall::render_block(&selected, &recent))
    }

    /// Keyword search for the model's `search_memory` tool and the CLI.
    /// Covers the same memories recall can see: the project's and the global
    /// ones. `scope` and `category` narrow the result; `limit` caps it.
    pub async fn search(
        &self,
        project: &ProjectIdentity,
        query: &str,
        scope: SearchScope,
        category: Option<MemoryCategory>,
        limit: usize,
    ) -> Result<Vec<Memory>> {
        let Some(expr) = recall::fts_match_expr(query, None) else {
            return Ok(Vec::new());
        };
        let now = Utc::now();
        let hits = self
            .repo
            .fts_search(&expr, &project.id, now, KEYWORD_LIMIT)
            .await?;
        Ok(hits
            .into_iter()
            .map(|(memory, _)| memory)
            .filter(|m| match scope {
                SearchScope::All => true,
                SearchScope::Global => matches!(m.scope, MemoryScope::Global),
                SearchScope::Project => matches!(m.scope, MemoryScope::Project(_)),
            })
            .filter(|m| category.is_none_or(|c| m.category == c))
            .take(limit)
            .collect())
    }

    /// One memory by its full id.
    pub async fn get(&self, id: Uuid) -> Result<Option<Memory>> {
        self.repo.get(id).await
    }

    /// Delete a memory for good. `actor` records who asked: "user" or "model".
    pub async fn forget(&self, id: Uuid, actor: &str) -> Result<()> {
        self.repo.delete(id, actor, Utc::now()).await
    }

    /// The project's memories and the global ones, for `crustly memory list`.
    pub async fn list(
        &self,
        project: &ProjectIdentity,
        include_inactive: bool,
    ) -> Result<Vec<Memory>> {
        self.repo
            .list_for_project(&project.id, include_inactive)
            .await
    }

    /// Every memory in the database, for export.
    pub async fn all(&self) -> Result<Vec<Memory>> {
        self.repo.list_all().await
    }

    /// The audit trail of one memory: (event, actor, time).
    pub async fn events(&self, id: Uuid) -> Result<Vec<(String, String, chrono::DateTime<Utc>)>> {
        self.repo.events(id).await
    }

    /// Known projects: (id, root, name, active memory count).
    pub async fn projects(&self) -> Result<Vec<(ProjectId, String, String, i64)>> {
        self.repo.projects().await
    }

    /// Change the text of a memory. The new text is validated, scanned and
    /// checked for a duplicate in the same scope, like a save.
    pub async fn edit(
        &self,
        id: Uuid,
        text: &str,
        actor: &str,
    ) -> Result<std::result::Result<(), RejectReason>> {
        let Some(memory) = self.repo.get(id).await? else {
            return Ok(Err(RejectReason::NotFound));
        };
        if let Err(reason) = validate(text) {
            return Ok(Err(reason));
        }
        let content = normalize(text);
        if let Some(rule) = secret_scan::scan(&content) {
            return Ok(Err(RejectReason::Secret { rule }));
        }
        let hash = content_hash(&content);
        if let Some(other) = self.repo.find_by_hash(&memory.scope, &hash).await? {
            if other.id != id {
                return Ok(Err(RejectReason::Duplicate));
            }
        }
        self.repo
            .update_content(id, &content, &hash, actor, Utc::now())
            .await?;
        Ok(Ok(()))
    }

    /// Make a project memory global. Fails with a reason when it already is,
    /// or when the same text is already a global memory.
    pub async fn promote(&self, id: Uuid) -> Result<std::result::Result<(), RejectReason>> {
        let Some(memory) = self.repo.get(id).await? else {
            return Ok(Err(RejectReason::NotFound));
        };
        if matches!(memory.scope, MemoryScope::Global) {
            return Ok(Err(RejectReason::ScopeMismatch));
        }
        let hash = content_hash(&memory.content);
        if self
            .repo
            .find_by_hash(&MemoryScope::Global, &hash)
            .await?
            .is_some()
        {
            return Ok(Err(RejectReason::Duplicate));
        }
        self.repo.promote(id, "user", Utc::now()).await?;
        Ok(Ok(()))
    }

    /// Move the memories of another project's root to `project`. Returns the
    /// number moved.
    pub async fn relink(&self, old_id: &ProjectId, project: &ProjectIdentity) -> Result<u64> {
        self.repo.upsert_project(project, Utc::now()).await?;
        self.repo.relink_project(old_id, &project.id).await
    }

    /// Memories whose id starts with `prefix`, for the short ids shown to users.
    pub async fn find(&self, prefix: &str) -> Result<Vec<Memory>> {
        self.repo.find_by_prefix(prefix).await
    }

    /// Notices of saves since the last call, for the user to see.
    pub fn take_notices(&self) -> Vec<String> {
        match self.notices.lock() {
            Ok(mut notices) => std::mem::take(&mut *notices),
            Err(_) => Vec::new(),
        }
    }

    fn notify(&self, scope: &MemoryScope, category: &str, id: Uuid, content: &str) {
        let label = match scope {
            MemoryScope::Global => "global",
            MemoryScope::Project(_) => "project",
        };
        let short = id.simple().to_string();
        let line = format!(
            "Remembered ({label} · {category}): {content} [m:{}]",
            &short[..8]
        );
        if let Ok(mut notices) = self.notices.lock() {
            notices.push(line);
        }
    }
}

/// The memory_events actor for a source.
fn actor_for(source: MemorySource) -> &'static str {
    match source {
        MemorySource::User => "user",
        MemorySource::Tool => "model",
        MemorySource::Extracted => "extractor",
        MemorySource::Imported => "import",
        MemorySource::Consolidated => "consolidator",
    }
}
