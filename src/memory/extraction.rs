//! Running extraction: reading the new messages of a session, asking the
//! model for memories, and saving what passes (docs/improve-memory.md §12.1,
//! §12.4, §12.6; tasks P3.4 and P3.6).
//!
//! Extraction is off unless `[memory.extraction] enabled` is set. It runs in
//! the background from the store, one run per session at a time, and never
//! blocks a request. A failed run (the provider fails, the answer is
//! unusable, or a save fails) leaves the watermark where it was; after three
//! failures on the same range the watermark moves on, so one bad
//! conversation cannot block extraction forever. A run reads at most 40
//! turns, so a long backlog is read over several runs.

use super::extract::{self, Turn};
use super::model::{MemoryScope, MemorySource, MemoryStatus, NewMemory, SaveOutcome};
use super::project::ProjectIdentity;
use super::MemoryStore;
use crate::llm::provider::{ContentBlock, LLMRequest, Message, Provider};
use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;

/// Failed runs on one range before the watermark moves on.
const MAX_FAILURES: i64 = 3;

/// Saved memories a run shows the model, so it can update rather than repeat.
const EXISTING_SHOWN: usize = 10;

/// Turns (user messages and answers) one run reads. A longer backlog, as when
/// extraction is turned on in a long session, is read over several runs, so
/// one prompt cannot outgrow the model's context window.
const MAX_TURNS_PER_RUN: usize = 40;

/// The model and provider that extraction uses (provider "inherit").
pub struct ExtractionHandle {
    pub provider: Arc<dyn Provider>,
    pub model: String,
}

impl MemoryStore {
    /// Turn on extraction with the active provider, when the config enables it.
    pub fn with_extraction(mut self, provider: Arc<dyn Provider>, model: String) -> Self {
        if self.config.extraction.enabled {
            self.extraction = Some(ExtractionHandle { provider, model });
        }
        self
    }

    /// Start an extraction run for `session` in the background, unless one is
    /// already running for it or extraction is off. Returns at once.
    pub fn trigger(
        self: &Arc<Self>,
        session: Uuid,
        project: ProjectIdentity,
        session_end: bool,
        force: bool,
    ) {
        if self.extraction.is_none() {
            return;
        }
        if let Ok(mut running) = self.running.lock() {
            if !running.insert(session) {
                return;
            }
        }
        let store = self.clone();
        tokio::spawn(async move {
            if let Err(err) = store.extract(session, &project, session_end, force).await {
                tracing::warn!("memory extraction failed: {err:#}");
            }
            if let Ok(mut running) = store.running.lock() {
                running.remove(&session);
            }
        });
    }

    /// Whether extraction is on (enabled, with a model to call).
    pub fn extraction_on(&self) -> bool {
        self.extraction.is_some()
    }

    /// Whether `session` has user messages past its watermark, which a
    /// session-end run would read.
    pub async fn has_unread_turns(&self, session: Uuid) -> Result<bool> {
        let (last, _) = self.repo.extraction_state(session).await?.unwrap_or((0, 0));
        let rows = self.repo.turns_after(session, last).await?;
        Ok(rows.iter().any(|(_, role, _)| role == "user"))
    }

    /// The session-end run in the foreground, for when the TUI quits: its
    /// saves, and the episode that starts the next session's recent work,
    /// are made before the process exits. A background run on the same
    /// session is waited for first, so the two do not overlap.
    pub async fn finish_session(&self, session: Uuid, project: &ProjectIdentity) -> Result<usize> {
        if self.extraction.is_none() {
            return Ok(0);
        }
        loop {
            let free = self
                .running
                .lock()
                .map(|mut running| running.insert(session))
                .unwrap_or(true);
            if free {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        let result = self.extract(session, project, true, false).await;
        if let Ok(mut running) = self.running.lock() {
            running.remove(&session);
        }
        result
    }

    /// One extraction run now: returns how many memories were saved. `force`
    /// runs it even below the every-N count (before compaction). Does
    /// nothing when extraction is off or the session has no new messages to
    /// consider. Errors are returned, and the watermark is not moved.
    pub async fn extract(
        &self,
        session: Uuid,
        project: &ProjectIdentity,
        session_end: bool,
        force: bool,
    ) -> Result<usize> {
        let Some(handle) = self.extraction.as_ref() else {
            return Ok(0);
        };
        let now = Utc::now();
        let (last, failed) = self.repo.extraction_state(session).await?.unwrap_or((0, 0));
        let mut rows = self.repo.turns_after(session, last).await?;
        let new_users = rows.iter().filter(|(_, role, _)| role == "user").count();
        if !force
            && !extract::should_extract(
                new_users,
                self.config.extraction.every_n_user_messages,
                session_end,
            )
        {
            return Ok(0);
        }
        if rows.len() > MAX_TURNS_PER_RUN {
            if session_end {
                // The last run of the session: its episode should say how the
                // session ended, so it reads the newest turns.
                rows = rows.split_off(rows.len() - MAX_TURNS_PER_RUN);
            } else {
                // The oldest first; the watermark stops after them and the
                // next run reads on.
                rows.truncate(MAX_TURNS_PER_RUN);
            }
        }
        let Some(highest) = rows.iter().map(|(seq, _, _)| *seq).max() else {
            return Ok(0);
        };
        // Project memories need their project recorded; a run at session end
        // can come before any request did it.
        self.repo.upsert_project(project, now).await?;

        let turns: Vec<Turn> = rows
            .into_iter()
            .map(|(_, role, text)| Turn { role, text })
            .collect();
        let transcript = extract::transcript(&turns);
        let existing: Vec<(Uuid, String)> = self
            .search(
                project,
                &transcript,
                super::SearchScope::All,
                None,
                EXISTING_SHOWN,
            )
            .await?
            .into_iter()
            .map(|m| (m.id, m.content))
            .collect();
        let prompt = extract::build_prompt(&existing, &transcript, session_end);

        let model = if self.config.extraction.model.is_empty() {
            handle.model.clone()
        } else {
            self.config.extraction.model.clone()
        };
        let request = LLMRequest::new(model, vec![Message::user(prompt)]).with_temperature(0.0);
        let response = match handle.provider.complete(request).await {
            Ok(response) => response,
            Err(err) => {
                self.record_failure(session, last, highest, failed, now)
                    .await?;
                return Err(err).context("the model did not answer the extraction request");
            }
        };
        let answer: String = response
            .content
            .iter()
            .filter_map(|block| match block {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect();

        let config = &self.config.extraction;
        let candidates = match extract::parse(
            &answer,
            session_end,
            config.min_importance,
            config.max_per_run,
        ) {
            Ok(candidates) => candidates,
            Err(_) => {
                // Unusable answer: count it, and move on after the last try.
                self.record_failure(session, last, highest, failed, now)
                    .await?;
                return Ok(0);
            }
        };

        let mut saved = 0;
        for candidate in candidates {
            let scope = if candidate.global {
                MemoryScope::Global
            } else {
                MemoryScope::Project(project.id.clone())
            };
            // Review holds everything but episodes, which are a record of the
            // session rather than a claim to check.
            let status = if config.review && !candidate.episode {
                MemoryStatus::Pending
            } else {
                MemoryStatus::Active
            };
            let outcome = match self
                .save_with(
                    NewMemory {
                        content: candidate.content,
                        scope,
                        category: candidate.category,
                        source: MemorySource::Extracted,
                        importance: candidate.importance,
                        replaces: candidate.replaces,
                        session_id: Some(session),
                        expires_at: candidate
                            .expires_in_days
                            .map(|days| now + Duration::days(days)),
                    },
                    status,
                    candidate.evidence_user,
                )
                .await
            {
                Ok(outcome) => outcome,
                Err(err) => {
                    // Saves made before the error are kept; a retry finds them
                    // as exact duplicates.
                    self.record_failure(session, last, highest, failed, now)
                        .await?;
                    return Err(err).context("saving an extracted memory");
                }
            };
            if matches!(outcome, SaveOutcome::Created(_)) {
                saved += 1;
            }
        }
        self.repo
            .set_extraction_state(session, highest, 0, now)
            .await?;
        Ok(saved)
    }

    /// Count a failed run on the range after `last`. After `MAX_FAILURES`
    /// failures the watermark moves to `highest` anyway, so one range that
    /// always fails (too long for the model, an answer it cannot give) does
    /// not cost a model call on every request.
    async fn record_failure(
        &self,
        session: Uuid,
        last: i64,
        highest: i64,
        failed: i64,
        now: chrono::DateTime<Utc>,
    ) -> Result<()> {
        if failed + 1 >= MAX_FAILURES {
            tracing::warn!(
                "memory extraction failed {MAX_FAILURES} times on session {session}; skipping its messages up to {highest}"
            );
            self.repo
                .set_extraction_state(session, highest, 0, now)
                .await
        } else {
            self.repo
                .set_extraction_state(session, last, failed + 1, now)
                .await
        }
    }

    /// Approve a pending memory, so it is recalled from now on. A memory it
    /// was proposed to replace is superseded by it now, if that memory is
    /// still active in the same scope.
    pub async fn approve(
        &self,
        id: Uuid,
    ) -> Result<std::result::Result<(), super::model::RejectReason>> {
        let Some(memory) = self.repo.get(id).await? else {
            return Ok(Err(super::model::RejectReason::NotFound));
        };
        if memory.status != MemoryStatus::Pending {
            return Ok(Err(super::model::RejectReason::NotFound));
        }
        let now = Utc::now();
        let target = self.repo.pending_replaces(id).await?;
        self.repo.approve(id, now).await?;
        if let Some(old_id) = target {
            if let Some(old) = self.repo.get(old_id).await? {
                if old.status == MemoryStatus::Active && old.scope == memory.scope {
                    self.repo.supersede(old.id, id, "user", now).await?;
                }
            }
        }
        self.changed();
        // The embedding worker skips pending memories; now it is active.
        self.queue_embedding(id);
        Ok(Ok(()))
    }

    /// Pending memories of `project`, waiting for review.
    pub async fn pending(&self, project: &ProjectIdentity) -> Result<Vec<super::model::Memory>> {
        self.repo.pending_for(&project.id).await
    }
}
