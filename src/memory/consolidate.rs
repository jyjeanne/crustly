//! Consolidation: expiry, episode retention, merging near-duplicates, lessons
//! from episodes, and the staleness list (docs/improve-memory.md §8.3, §13;
//! tasks P4.5 to P4.7).
//!
//! Consolidation never deletes. It archives or supersedes, and keeps history.
//! Without `apply` it only reports. Changes to memories the user saved are
//! made only when the caller applies (the CLI's `--yes`).

use super::model::{Memory, MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome};
use super::project::ProjectIdentity;
use super::semantic::near_duplicate_to_archive;
use super::vector::similarity;
use super::MemoryStore;
use crate::llm::provider::{ContentBlock, LLMRequest, Message};
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashSet;
use uuid::Uuid;

/// Lower bound of the merge-candidate band; above it a pair is a duplicate
/// and was already archived on save (§13.1).
pub const MERGE_LOW: f32 = 0.80;
/// Upper bound of the merge-candidate band.
pub const MERGE_HIGH: f32 = 0.92;

/// A memory not recalled for this long, and older than it, is listed for review.
const STALE_DAYS: i64 = 365;

/// Episodes needed before they are written up as one lesson.
const EPISODES_FOR_LESSON: usize = 3;

/// Most episodes one lesson summarises.
const EPISODES_PER_LESSON: usize = 10;

/// What consolidation found, and what it changed when applied.
#[derive(Debug, Default)]
pub struct ConsolidationReport {
    pub expired: usize,
    pub episodes_archived: usize,
    /// Near-duplicate pairs: (superseded, kept, similarity).
    pub merge_candidates: Vec<(Memory, Memory, f32)>,
    pub merged: usize,
    /// Facts and lessons not used for a year, for the user to keep or forget.
    pub stale: Vec<Memory>,
    pub lessons_written: usize,
}

/// The similarity of two stored vectors, when both exist and match in size.
fn pair_similarity(a: &[f32], b: &[f32]) -> Option<f32> {
    similarity(a, b).ok()
}

/// Whether a memory is stale: a fact or lesson, created and last used long ago.
pub fn is_stale(memory: &Memory, now: DateTime<Utc>) -> bool {
    let cutoff = now - Duration::days(STALE_DAYS);
    matches!(
        memory.category,
        MemoryCategory::Fact | MemoryCategory::Lesson
    ) && memory.created_at < cutoff
        && memory.last_recalled_at.unwrap_or(memory.created_at) < cutoff
}

/// Whether an episode has outlived the retention period.
pub fn is_expired_episode(memory: &Memory, retention_days: i64, now: DateTime<Utc>) -> bool {
    memory.category == MemoryCategory::Episode
        && memory.created_at < now - Duration::days(retention_days)
}

impl MemoryStore {
    /// Run consolidation for `project`. Returns what was found; with `apply`
    /// it also archives, merges and writes lessons.
    pub async fn consolidate(
        &self,
        project: &ProjectIdentity,
        apply: bool,
    ) -> Result<ConsolidationReport> {
        let now = Utc::now();
        let mut report = ConsolidationReport::default();
        let active = self.repo.list_for_project(&project.id, false).await?;
        // What this run archives (or would): the later steps leave it alone,
        // so a report without `apply` matches what `apply` does.
        let mut gone: HashSet<Uuid> = HashSet::new();

        // Expiry: a memory past its date is no longer true for the user.
        for memory in active
            .iter()
            .filter(|m| m.expires_at.is_some_and(|t| t <= now))
        {
            report.expired += 1;
            gone.insert(memory.id);
            if apply {
                self.repo.archive(memory.id, "consolidator", now).await?;
            }
        }

        // Episodes past the retention period are archived.
        let retention = self.config.lifecycle.episode_retention_days;
        let old_episodes: Vec<&Memory> = active
            .iter()
            .filter(|m| !gone.contains(&m.id) && is_expired_episode(m, retention, now))
            .collect();
        for memory in old_episodes {
            report.episodes_archived += 1;
            gone.insert(memory.id);
            if apply {
                self.repo.archive(memory.id, "consolidator", now).await?;
            }
        }

        // Staleness: listed only, never archived for disuse (§13.4).
        report.stale = active
            .iter()
            .filter(|m| is_stale(m, now))
            .cloned()
            .collect();

        // Near-duplicates: pairs in the merge band. Which one is superseded
        // follows the near-duplicate rules: same category only, never
        // episodes, the user's copy first, then the newer one.
        if let Some(model) = self.embedding_model() {
            let vectors = self.repo.vectors_for(&project.id, &model, now).await?;
            let by_id: std::collections::HashMap<Uuid, &Memory> = active
                .iter()
                .filter(|m| !gone.contains(&m.id))
                .map(|m| (m.id, m))
                .collect();
            let mut merged_already: HashSet<Uuid> = HashSet::new();
            for (i, (a_id, a_vec)) in vectors.iter().enumerate() {
                for (b_id, b_vec) in vectors.iter().skip(i + 1) {
                    let (Some(a), Some(b)) = (by_id.get(a_id), by_id.get(b_id)) else {
                        continue;
                    };
                    if a.scope != b.scope || !matches!(a.scope, MemoryScope::Project(_)) {
                        continue;
                    }
                    let Some(score) = pair_similarity(a_vec, b_vec) else {
                        continue;
                    };
                    if !(MERGE_LOW..MERGE_HIGH).contains(&score) {
                        continue;
                    }
                    let Some(loser) = near_duplicate_to_archive(a, b) else {
                        continue;
                    };
                    let (superseded, kept) = if loser.id == a.id { (*a, *b) } else { (*b, *a) };
                    report
                        .merge_candidates
                        .push((superseded.clone(), kept.clone(), score));
                    if apply
                        && !merged_already.contains(&superseded.id)
                        && !merged_already.contains(&kept.id)
                    {
                        self.repo
                            .supersede(superseded.id, kept.id, "consolidator", now)
                            .await?;
                        merged_already.insert(superseded.id);
                        report.merged += 1;
                    }
                }
            }
        }

        // Episodes become one lesson once there are enough of them.
        if apply {
            report.lessons_written = self.lesson_from_episodes(project, &active, &gone).await?;
        }
        Ok(report)
    }

    /// The model name of the configured embedder, if there is one.
    fn embedding_model(&self) -> Option<String> {
        self.embed
            .as_ref()
            .map(|h| h.embedder.model_id().to_string())
    }

    /// Write one lesson from the active episodes that do not yet feed one.
    /// Needs extraction on and its model; without them, nothing is written.
    /// `gone` holds what this run archived, which is left out.
    async fn lesson_from_episodes(
        &self,
        project: &ProjectIdentity,
        active: &[Memory],
        gone: &HashSet<Uuid>,
    ) -> Result<usize> {
        if !self.config.extraction.enabled {
            return Ok(0);
        }
        let Some(handle) = self.extraction.as_ref() else {
            return Ok(0);
        };
        // An episode feeds one lesson at most; without this, every run wrote
        // the same lesson again in other words.
        let folded = self.repo.folded_sources(&project.id).await?;
        let mut episodes: Vec<&Memory> = active
            .iter()
            .filter(|m| {
                m.category == MemoryCategory::Episode
                    && !gone.contains(&m.id)
                    && !folded.contains(&m.id)
            })
            .collect();
        if episodes.len() < EPISODES_FOR_LESSON {
            return Ok(0);
        }
        episodes.sort_by_key(|m| m.created_at);
        episodes.truncate(EPISODES_PER_LESSON);
        let listing: String = episodes
            .iter()
            .map(|m| format!("- {}\n", m.content))
            .collect();
        let prompt = format!(
            "These are summaries of several coding sessions on one project.\n\
             Write one lesson that generalises what they show: one standalone sentence of at \
             most 300 characters, useful in future sessions, not a summary of any one session.\n\
             Return only JSON: {{\"lesson\": \"...\"}}\n\nSESSIONS:\n{listing}"
        );
        let model = if self.config.extraction.model.is_empty() {
            handle.model.clone()
        } else {
            self.config.extraction.model.clone()
        };
        let response = handle
            .provider
            .complete(LLMRequest::new(model, vec![Message::user(prompt)]).with_temperature(0.0))
            .await?;
        let answer: String = response
            .content
            .iter()
            .filter_map(|b| match b {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        let Some(lesson) = parse_lesson(&answer) else {
            return Ok(0);
        };
        let outcome = self
            .save(NewMemory {
                content: lesson,
                scope: MemoryScope::Project(project.id.clone()),
                category: MemoryCategory::Lesson,
                source: MemorySource::Consolidated,
                importance: 3,
                replaces: None,
                session_id: None,
                expires_at: None,
            })
            .await?;
        if let SaveOutcome::Created(id) = outcome {
            let sources: Vec<Uuid> = episodes.iter().map(|m| m.id).collect();
            self.repo.add_sources(id, &sources).await?;
            return Ok(1);
        }
        Ok(0)
    }
}

/// The lesson in a model's answer: the `lesson` field of the first JSON object.
pub fn parse_lesson(answer: &str) -> Option<String> {
    let start = answer.find('{')?;
    let end = answer.rfind('}')?;
    let value: serde_json::Value = serde_json::from_str(answer.get(start..=end)?).ok()?;
    let text = value.get("lesson")?.as_str()?.trim().to_string();
    (!text.is_empty() && text.chars().count() <= 300).then_some(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::model::MemoryStatus;

    fn memory(
        category: MemoryCategory,
        created: DateTime<Utc>,
        recalled: Option<DateTime<Utc>>,
    ) -> Memory {
        Memory {
            id: Uuid::new_v4(),
            scope: MemoryScope::Project("p".into()),
            category,
            content: "x".into(),
            source: MemorySource::User,
            importance: 3,
            status: MemoryStatus::Active,
            superseded_by: None,
            created_at: created,
            updated_at: created,
            last_recalled_at: recalled,
            recall_count: 0,
            expires_at: None,
        }
    }

    #[test]
    fn a_year_unused_fact_is_stale_but_a_preference_never_is() {
        let now = DateTime::from_timestamp(1_800_000_000, 0).unwrap();
        let old = now - Duration::days(400);
        assert!(is_stale(&memory(MemoryCategory::Fact, old, None), now));
        assert!(!is_stale(
            &memory(MemoryCategory::Preference, old, None),
            now
        ));
        assert!(
            !is_stale(&memory(MemoryCategory::Fact, old, Some(now)), now),
            "recently used"
        );
    }

    #[test]
    fn episodes_expire_after_the_retention_period() {
        let now = DateTime::from_timestamp(1_800_000_000, 0).unwrap();
        assert!(is_expired_episode(
            &memory(MemoryCategory::Episode, now - Duration::days(200), None),
            180,
            now
        ));
        assert!(!is_expired_episode(
            &memory(MemoryCategory::Episode, now - Duration::days(10), None),
            180,
            now
        ));
        assert!(!is_expired_episode(
            &memory(MemoryCategory::Fact, now - Duration::days(200), None),
            180,
            now
        ));
    }

    #[test]
    fn a_lesson_is_read_from_the_first_json_object() {
        assert_eq!(
            parse_lesson(r#"sure: {"lesson": "Avoid blocking calls in async code."}"#).as_deref(),
            Some("Avoid blocking calls in async code.")
        );
        assert!(parse_lesson(r#"{"lesson": ""}"#).is_none());
        assert!(parse_lesson("no json").is_none());
    }
}
