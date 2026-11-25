//! Memories anchored to code concepts (docs/improve-memory.md §23.3 and
//! §23.4; tasks P6.5 to P6.7).
//!
//! A project memory that names a function or a type (`connect_pool`,
//! `Database::run_migrations`) is anchored to that concept of the okf-rs
//! bundle, at most three of them. Recall then finds it when a request names
//! the concept, or one of its direct callers or callees, and a tool call on
//! the file that defines it brings it in like a path-scoped memory. When the
//! concept disappears from a newer bundle, the memory is listed for review.

use super::code_knowledge::{CodeKnowledge, Detection, MAX_ANCHORS, MAX_NEIGHBOUR_SOURCES};
use super::model::{Memory, MemoryScope, MemoryStatus, ProjectId};
use super::project::ProjectIdentity;
use super::MemoryStore;
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

/// A memory whose anchored concept is gone from a bundle newer than the
/// anchor (§23.4).
#[derive(Debug, Clone)]
pub struct Drift {
    pub memory: Memory,
    pub concept: String,
    /// The closest concept by name, to re-anchor to.
    pub closest: Option<String>,
}

/// Memories a request reaches through code knowledge.
#[derive(Debug, Default)]
pub(crate) struct AnchorHits {
    /// Anchored to a concept the request names (K = 1).
    pub direct: HashSet<Uuid>,
    /// Anchored to a direct caller or callee of one (K = 0.5).
    pub neighbours: HashSet<Uuid>,
}

impl MemoryStore {
    /// Use code knowledge as detected for this project: off with a reason,
    /// or a handle (started separately, see `start_code_knowledge`).
    pub fn with_code_knowledge(mut self, detection: Detection) -> Self {
        match detection {
            Detection::Off(reason) => {
                self.code = None;
                self.code_off = Some(reason);
            }
            Detection::Available(code) => {
                self.code = Some(Arc::new(code));
                self.code_off = None;
            }
        }
        self
    }

    /// The code knowledge handle, when the feature is on.
    pub fn code_knowledge(&self) -> Option<&Arc<CodeKnowledge>> {
        self.code.as_ref()
    }

    /// Code knowledge as `crustly memory status` reports it.
    pub fn code_knowledge_status(&self) -> String {
        match (&self.code, &self.code_off) {
            (Some(code), _) => code.status_line(),
            (None, Some(reason)) => format!("off: {reason}"),
            (None, None) => "off".to_string(),
        }
    }

    /// Start okf-mcp in the background (a chat or a run): requests before it
    /// is ready go without code knowledge.
    pub fn start_code_knowledge_in_background(&self) {
        if let Some(code) = &self.code {
            code.spawn_start();
        }
    }

    /// Start okf-mcp and wait until it is ready (CLI commands). Returns
    /// whether code knowledge can be used.
    pub async fn start_code_knowledge(&self) -> bool {
        match &self.code {
            Some(code) => code.start().await,
            None => false,
        }
    }

    /// The ready handle, if any.
    fn ready_code(&self) -> Option<&Arc<CodeKnowledge>> {
        self.code.as_ref().filter(|code| code.is_ready())
    }

    /// Anchor a project memory to the concepts its text names (§23.3):
    /// exact, unambiguous name matches only, at most three. Replaces the
    /// memory's anchors when some are found; leaves them alone otherwise.
    /// Returns the anchors set.
    pub async fn anchor_automatically(&self, id: Uuid) -> Result<Vec<String>> {
        let Some(code) = self.ready_code() else {
            return Ok(Vec::new());
        };
        let Some(memory) = self.repo.get(id).await? else {
            return Ok(Vec::new());
        };
        if !matches!(memory.scope, MemoryScope::Project(_)) {
            return Ok(Vec::new());
        }
        let concepts = code.resolve_text(&memory.content).await;
        if !concepts.is_empty() {
            let at = code.bundle_time().unwrap_or_else(Utc::now);
            self.repo.set_anchors(id, &concepts, at).await?;
        }
        Ok(concepts)
    }

    /// Set a project memory's anchors by hand. Each concept must exist when
    /// code knowledge can check it. `Err` holds the reason for a refusal.
    pub async fn set_anchors(
        &self,
        id: Uuid,
        concepts: &[String],
    ) -> Result<std::result::Result<(), String>> {
        let Some(memory) = self.repo.get(id).await? else {
            return Ok(Err("no such memory".to_string()));
        };
        if !matches!(memory.scope, MemoryScope::Project(_)) && !concepts.is_empty() {
            return Ok(Err("only project memories are anchored to code".to_string()));
        }
        if concepts.len() > MAX_ANCHORS {
            return Ok(Err(format!("at most {MAX_ANCHORS} anchors")));
        }
        let code = self.ready_code();
        if let Some(code) = code {
            for concept in concepts {
                if code.exists(concept).await == Some(false) {
                    let hint = code
                        .closest(concept)
                        .await
                        .map(|c| format!(" (closest: {c})"))
                        .unwrap_or_default();
                    return Ok(Err(format!("no concept `{concept}` in the bundle{hint}")));
                }
            }
        }
        let at = code.and_then(|c| c.bundle_time()).unwrap_or_else(Utc::now);
        self.repo.set_anchors(id, concepts, at).await?;
        Ok(Ok(()))
    }

    /// A memory's anchors: (concept id, when the anchor was made).
    pub async fn anchors(&self, id: Uuid) -> Result<Vec<(String, DateTime<Utc>)>> {
        self.repo.anchors_of(id).await
    }

    /// The memories of `project_id` anchored to any of `concepts`.
    async fn anchored_to(
        &self,
        project_id: &ProjectId,
        concepts: &HashSet<String>,
    ) -> Result<HashSet<Uuid>> {
        if concepts.is_empty() {
            return Ok(HashSet::new());
        }
        Ok(self
            .repo
            .anchors_for(project_id, Utc::now())
            .await?
            .into_iter()
            .filter(|(_, concept, _)| concepts.contains(concept))
            .map(|(id, _, _)| id)
            .collect())
    }

    /// Memories `message` reaches through code knowledge (§23.3, recall 1
    /// and 2): anchored to a concept it names, or to a direct caller or
    /// callee of one. Nothing when the project has no anchors, so a request
    /// costs no okf-mcp query until some memory is anchored.
    pub(crate) async fn anchor_hits(&self, project_id: &ProjectId, message: &str) -> AnchorHits {
        let Some(code) = self.ready_code() else {
            return AnchorHits::default();
        };
        match self.repo.anchors_for(project_id, Utc::now()).await {
            Ok(anchors) if !anchors.is_empty() => {}
            _ => return AnchorHits::default(),
        }
        let named = code.resolve_text(message).await;
        let mut around: HashSet<String> = HashSet::new();
        for concept in named.iter().take(MAX_NEIGHBOUR_SOURCES) {
            around.extend(code.neighbours(concept).await);
        }
        let named: HashSet<String> = named.into_iter().collect();
        let direct = self
            .anchored_to(project_id, &named)
            .await
            .unwrap_or_default();
        let neighbours = self
            .anchored_to(project_id, &around)
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|id| !direct.contains(id))
            .collect();
        AnchorHits { direct, neighbours }
    }

    /// Memories anchored to a concept defined in `relative` (a path relative
    /// to the project root), for the tool-result hook (§23.3, recall 3).
    pub(crate) async fn anchored_in_file(
        &self,
        project_id: &ProjectId,
        relative: &str,
    ) -> HashSet<Uuid> {
        let Some(code) = self.ready_code() else {
            return HashSet::new();
        };
        match self.repo.anchors_for(project_id, Utc::now()).await {
            Ok(anchors) if !anchors.is_empty() => {}
            _ => return HashSet::new(),
        }
        let concepts: HashSet<String> = code.file_concepts(relative).await.into_iter().collect();
        self.anchored_to(project_id, &concepts)
            .await
            .unwrap_or_default()
    }

    /// Where the code of each anchored memory in `memories` is now
    /// (`file#Lstart-Lend`, from its first anchor), for the recall block.
    pub(crate) async fn anchor_locations(
        &self,
        project_id: &ProjectId,
        memories: &[Memory],
    ) -> HashMap<Uuid, String> {
        let mut locations = HashMap::new();
        let Some(code) = self.ready_code() else {
            return locations;
        };
        let Ok(anchors) = self.repo.anchors_for(project_id, Utc::now()).await else {
            return locations;
        };
        for memory in memories {
            // Anchors are listed by memory, then concept: the first is kept.
            let Some((_, concept, _)) = anchors.iter().find(|(id, _, _)| *id == memory.id) else {
                continue;
            };
            if let Some(location) = code.location(concept).await {
                locations.insert(memory.id, location);
            }
        }
        locations
    }

    /// Anchored memories whose concept is gone from a bundle newer than the
    /// anchor (§23.4). A concept missing from an older bundle is not counted:
    /// the bundle may simply be out of date. Changes nothing.
    pub async fn drifted(&self, project: &ProjectIdentity) -> Result<Vec<Drift>> {
        let Some(code) = self.ready_code() else {
            return Ok(Vec::new());
        };
        let Some(bundle_time) = code.bundle_time() else {
            return Ok(Vec::new());
        };
        let mut drifted = Vec::new();
        for (id, concept, anchored_at) in self.repo.anchors_for(&project.id, Utc::now()).await? {
            // Anchors keep whole seconds; so does this comparison, or the
            // very bundle an anchor was made from would look newer.
            if bundle_time.timestamp() <= anchored_at.timestamp() {
                continue;
            }
            if code.exists(&concept).await != Some(false) {
                continue;
            }
            let Some(memory) = self.repo.get(id).await? else {
                continue;
            };
            if memory.status != MemoryStatus::Active {
                continue;
            }
            let closest = code.closest(&concept).await;
            drifted.push(Drift {
                memory,
                concept,
                closest,
            });
        }
        Ok(drifted)
    }
}
