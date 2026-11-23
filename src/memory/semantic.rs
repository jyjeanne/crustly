//! Semantic recall: embedding memories in the background, searching them by
//! meaning, and the cached index they are searched through (docs/improve-memory.md
//! §6.4, §6.5, §9.4, §11.4; tasks P2.7, P2.8 wiring and P2.9).
//!
//! Saving a memory never waits for its vector. The memory is queued, a
//! background worker embeds it in batches, and until then keyword recall
//! still finds it. Failed embeddings are retried with backoff and otherwise
//! left for the next backfill.

use super::embed::{EmbedError, EmbedPurpose, Embedder};
use super::model::{Memory, ProjectId};
use super::vector_index::VectorIndex;
use crate::db::repository::MemoryRepository;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Queue capacity. A full queue drops the id; the backfill picks it up later.
const QUEUE_CAPACITY: usize = 256;

/// Nearest memories kept per request before the threshold and budget apply.
pub const SEMANTIC_CANDIDATES: usize = 20;

/// Two selected memories this similar are one memory told twice (§9.4).
pub const NEAR_DUPLICATE: f32 = 0.92;

/// How semantic search is run.
#[derive(Debug, Clone)]
pub struct EmbedRuntime {
    /// Longest wait for a query embedding; after it, keyword recall alone.
    pub query_timeout: Duration,
    /// Memories embedded per call.
    pub batch_size: usize,
    /// Cosine similarity below which a memory is not relevant.
    pub min_similarity: f32,
    /// Waits between retries of a failed embedding call.
    pub retry_delays: Vec<Duration>,
}

impl Default for EmbedRuntime {
    fn default() -> Self {
        Self {
            query_timeout: Duration::from_millis(1500),
            batch_size: 16,
            min_similarity: 0.35,
            retry_delays: vec![
                Duration::from_secs(1),
                Duration::from_secs(4),
                Duration::from_secs(16),
            ],
        }
    }
}

/// What the store holds once an embedder is configured.
pub struct EmbedHandle {
    pub embedder: Arc<dyn Embedder>,
    pub queue: mpsc::Sender<Uuid>,
    pub runtime: EmbedRuntime,
}

/// The index for one project and model, valid while the generation matches.
pub struct CachedIndex {
    pub project: ProjectId,
    pub generation: u64,
    pub index: Arc<VectorIndex>,
}

/// Embed `texts` with retries: the first attempt, then one more per delay.
pub async fn embed_with_retries(
    embedder: &dyn Embedder,
    purpose: EmbedPurpose,
    texts: &[String],
    retry_delays: &[Duration],
) -> Result<Vec<Vec<f32>>, EmbedError> {
    let mut attempt = 0;
    loop {
        match embedder.embed(purpose, texts).await {
            Ok(vectors) => return Ok(vectors),
            Err(err) => match retry_delays.get(attempt) {
                Some(delay) => {
                    tracing::debug!("embedding attempt {} failed, retrying: {err}", attempt + 1);
                    tokio::time::sleep(*delay).await;
                    attempt += 1;
                }
                None => return Err(err),
            },
        }
    }
}

/// Embed `memories` as documents and store the vectors. Bumps `generation`
/// so cached indexes are rebuilt. Returns how many were stored.
pub async fn embed_and_store(
    repo: &MemoryRepository,
    embedder: &dyn Embedder,
    memories: &[Memory],
    retry_delays: &[Duration],
    generation: &AtomicU64,
    now: chrono::DateTime<chrono::Utc>,
) -> Result<usize, EmbedError> {
    if memories.is_empty() {
        return Ok(0);
    }
    let texts: Vec<String> = memories.iter().map(|m| m.content.clone()).collect();
    let vectors =
        embed_with_retries(embedder, EmbedPurpose::Document, &texts, retry_delays).await?;
    let mut stored = 0;
    for (memory, vector) in memories.iter().zip(&vectors) {
        match repo
            .put_embedding(memory.id, embedder.model_id(), vector, now)
            .await
        {
            Ok(()) => stored += 1,
            Err(err) => tracing::warn!("could not store a memory vector: {err:#}"),
        }
    }
    generation.fetch_add(1, Ordering::SeqCst);
    if let Err(err) = archive_near_duplicates(repo, embedder.model_id(), memories, now).await {
        tracing::warn!("near-duplicate check failed: {err:#}");
    }
    Ok(stored)
}

/// Embed every active memory that lacks a vector for the model, in batches.
/// Stops at the first failure, which is left for the next run.
pub async fn backfill(
    repo: &MemoryRepository,
    embedder: &dyn Embedder,
    runtime: &EmbedRuntime,
    generation: &AtomicU64,
) -> Result<usize, EmbedError> {
    let mut total = 0;
    loop {
        let batch = repo
            .missing_embeddings(embedder.model_id(), runtime.batch_size as i64)
            .await
            .map_err(|e| EmbedError::Transport(format!("reading memories: {e:#}")))?;
        if batch.is_empty() {
            return Ok(total);
        }
        total += embed_and_store(
            repo,
            embedder,
            &batch,
            &runtime.retry_delays,
            generation,
            chrono::Utc::now(),
        )
        .await?;
    }
}

/// Start the background worker. It backfills first, then embeds queued
/// memories in batches until the queue closes.
pub fn spawn_worker(
    repo: MemoryRepository,
    embedder: Arc<dyn Embedder>,
    runtime: EmbedRuntime,
    generation: Arc<AtomicU64>,
    mut queue: mpsc::Receiver<Uuid>,
) {
    tokio::spawn(async move {
        if let Err(err) = backfill(&repo, embedder.as_ref(), &runtime, &generation).await {
            tracing::warn!("memory backfill stopped: {err}");
        }
        while let Some(first) = queue.recv().await {
            let mut ids = vec![first];
            while ids.len() < runtime.batch_size {
                match queue.try_recv() {
                    Ok(id) => ids.push(id),
                    Err(_) => break,
                }
            }
            let mut memories = Vec::new();
            for id in ids {
                // A memory forgotten while queued is simply skipped.
                if let Ok(Some(memory)) = repo.get(id).await {
                    if memory.status == super::model::MemoryStatus::Active {
                        memories.push(memory);
                    }
                }
            }
            if let Err(err) = embed_and_store(
                &repo,
                embedder.as_ref(),
                &memories,
                &runtime.retry_delays,
                &generation,
                chrono::Utc::now(),
            )
            .await
            {
                tracing::warn!("memory embedding failed, will retry at the next backfill: {err}");
            }
        }
    });
}

/// A new queue for the worker, and its sending end.
pub fn queue() -> (mpsc::Sender<Uuid>, mpsc::Receiver<Uuid>) {
    mpsc::channel(QUEUE_CAPACITY)
}

/// The index of `project`'s vectors for the embedder's model, rebuilt when
/// the generation has moved on. Vectors of the wrong size are left out.
pub async fn current_index(
    cache: &std::sync::Mutex<Option<CachedIndex>>,
    generation: &AtomicU64,
    repo: &MemoryRepository,
    project: &ProjectId,
    model: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<Arc<VectorIndex>> {
    let wanted = generation.load(Ordering::SeqCst);
    if let Ok(guard) = cache.lock() {
        if let Some(cached) = guard.as_ref() {
            if cached.project == *project
                && cached.generation == wanted
                && cached.index.model() == model
            {
                return Ok(cached.index.clone());
            }
        }
    }
    let entries = repo.vectors_for(project, model, now).await?;
    let dimensions = entries.first().map(|(_, v)| v.len()).unwrap_or(0);
    let entries: Vec<(Uuid, Vec<f32>)> = entries
        .into_iter()
        .filter(|(_, v)| v.len() == dimensions)
        .collect();
    let index = Arc::new(VectorIndex::from_entries(model, dimensions, entries)?);
    if let Ok(mut guard) = cache.lock() {
        *guard = Some(CachedIndex {
            project: project.clone(),
            generation: wanted,
            index: index.clone(),
        });
    }
    Ok(index)
}

/// Normalised similarity (§9.4): 0 at the threshold, 1 for an identical text.
pub fn normalise(cosine: f32, min_similarity: f32) -> f64 {
    let span = (1.0 - min_similarity).max(1e-6);
    f64::from((cosine - min_similarity) / span).clamp(0.0, 1.0)
}

/// Drop a memory when an earlier-kept one says the same thing (§9.4). The
/// order of `selected` decides which of the two stays.
pub fn drop_near_duplicates(selected: Vec<Memory>, index: &VectorIndex) -> Vec<Memory> {
    let mut kept: Vec<Memory> = Vec::new();
    for memory in selected {
        let repeats = kept.iter().any(|k| {
            index
                .similarity_between(&memory.id, &k.id)
                .is_some_and(|s| s >= NEAR_DUPLICATE)
        });
        if !repeats {
            kept.push(memory);
        }
    }
    kept
}

/// What a semantic search of one request found.
pub struct SemanticHits {
    /// Normalised similarity per memory, for the memories above the threshold.
    pub scores: std::collections::HashMap<Uuid, f64>,
    pub index: Arc<VectorIndex>,
}

/// Archive memories that repeat an older memory in the same project (P2.10).
/// The older one stays active, so a batch cannot archive both copies. Global
/// memories are not checked here: their vectors are looked up per project.
pub async fn archive_near_duplicates(
    repo: &MemoryRepository,
    model: &str,
    memories: &[Memory],
    now: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<usize> {
    use super::model::{MemoryScope, MemoryStatus};
    use super::vector::similarity;
    let mut archived = 0;
    for memory in memories {
        let MemoryScope::Project(project) = &memory.scope else {
            continue;
        };
        let vectors = repo.vectors_for(project, model, now).await?;
        let Some((_, mine)) = vectors.iter().find(|(id, _)| *id == memory.id) else {
            continue;
        };
        for (other_id, theirs) in &vectors {
            if *other_id == memory.id {
                continue;
            }
            let Ok(score) = similarity(mine, theirs) else {
                continue;
            };
            if score < NEAR_DUPLICATE {
                continue;
            }
            let Some(other) = repo.get(*other_id).await? else {
                continue;
            };
            let older = (other.created_at, other.id.to_string())
                < (memory.created_at, memory.id.to_string());
            if other.scope == memory.scope && other.status == MemoryStatus::Active && older {
                repo.archive(memory.id, "consolidator", now).await?;
                archived += 1;
                break;
            }
        }
    }
    Ok(archived)
}
