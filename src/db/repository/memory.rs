//! SQL for long-term memory (docs/improve-memory.md §6.2, §8, §9.2).
//!
//! This layer does no policy: validation, secret scanning and reinforcement
//! rules live in `crate::memory`. It only reads and writes rows, and keeps the
//! memory_events audit trail in step with every change.

use crate::db::retry::{retry_db_sqlx, DbRetryConfig};
use crate::memory::model::{
    Memory, MemoryCategory, MemoryScope, MemorySource, MemoryStatus, ProjectId,
};
use crate::memory::project::ProjectIdentity;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

/// Columns read back for a `Memory`, in the order `row_to_memory` expects.
const MEMORY_COLUMNS: &str = "id, scope, project_id, category, content, source, importance, \
     status, superseded_by, created_at, updated_at, last_recalled_at, recall_count, expires_at";

/// A memory row to insert, already validated and hashed.
#[derive(Debug, Clone)]
pub struct InsertMemory {
    pub id: Uuid,
    pub scope: MemoryScope,
    pub category: MemoryCategory,
    pub content: String,
    pub content_hash: String,
    pub source: MemorySource,
    pub importance: u8,
    pub status: MemoryStatus,
    pub session_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub now: DateTime<Utc>,
}

pub struct MemoryRepository {
    pool: SqlitePool,
}

impl MemoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// A handle to the same database, for background work.
    pub fn pool(&self) -> SqlitePool {
        self.pool.clone()
    }

    // ── P1.8: writes ─────────────────────────────────────────────────────

    /// Record the project, or refresh its name, path and remote.
    pub async fn upsert_project(
        &self,
        project: &ProjectIdentity,
        now: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO memory_projects (id, root_path, name, remote_url, created_at, last_seen_at) \
             VALUES (?, ?, ?, ?, ?, ?) \
             ON CONFLICT(id) DO UPDATE SET root_path = excluded.root_path, \
             name = excluded.name, remote_url = excluded.remote_url, \
             last_seen_at = excluded.last_seen_at",
        )
        .bind(&project.id)
        .bind(project.root.to_string_lossy().as_ref())
        .bind(&project.name)
        .bind(&project.remote_url)
        .bind(now.timestamp())
        .bind(now.timestamp())
        .execute(&self.pool)
        .await
        .context("upserting memory project")?;
        Ok(())
    }

    /// Insert a memory and its `created` event in one transaction. `actor` is
    /// one of the memory_events actors: user, model, extractor, consolidator
    /// or import.
    pub async fn insert(&self, memory: &InsertMemory, actor: &str) -> Result<()> {
        let (scope, project_id) = scope_columns(&memory.scope);
        // The whole transaction is retried on a busy lock. A deferred
        // transaction that reads, then writes, can get SQLITE_BUSY at once
        // without waiting, so `busy_timeout` alone does not cover it.
        let (scope, project_id) = (&scope, &project_id);
        retry_db_sqlx(
            || async move {
                let mut tx = self.pool.begin().await?;
                sqlx::query(
                    "INSERT INTO memories (id, scope, project_id, category, content, content_hash, \
                     source, source_session_id, importance, status, created_at, updated_at, expires_at) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(memory.id.to_string())
                .bind(*scope)
                .bind(*project_id)
                .bind(memory.category.as_str())
                .bind(&memory.content)
                .bind(&memory.content_hash)
                .bind(memory.source.as_str())
                .bind(memory.session_id.map(|s| s.to_string()))
                .bind(i64::from(memory.importance))
                .bind(memory.status.as_str())
                .bind(memory.now.timestamp())
                .bind(memory.now.timestamp())
                .bind(memory.expires_at.map(|t| t.timestamp()))
                .execute(&mut *tx)
                .await?;
                record_event(
                    &mut tx,
                    memory.id,
                    "created",
                    actor,
                    memory.session_id,
                    memory.now,
                )
                .await?;
                tx.commit().await
            },
            &DbRetryConfig::default(),
        )
        .await
        .context("inserting memory")?;
        Ok(())
    }

    /// The active or pending memory in `scope` with this hash, if any.
    pub async fn find_by_hash(&self, scope: &MemoryScope, hash: &str) -> Result<Option<Memory>> {
        let (_, project_id) = scope_columns(scope);
        let row = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories \
             WHERE content_hash = ? AND status IN ('active', 'pending') \
             AND ifnull(project_id, '') = ifnull(?, '') LIMIT 1"
        ))
        .bind(hash)
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await?;
        row.as_ref().map(row_to_memory).transpose()
    }

    /// Set `updated_at` to show the memory was seen again.
    /// Archive a memory: kept for history, never recalled, and the event
    /// records who archived it.
    pub async fn archive(&self, id: Uuid, actor: &str, now: DateTime<Utc>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("UPDATE memories SET status = 'archived', updated_at = ? WHERE id = ?")
            .bind(now.timestamp())
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;
        record_event(&mut tx, id, "archived", actor, None, now).await?;
        tx.commit().await?;
        Ok(())
    }

    // ── P3: extraction ───────────────────────────────────────────────────

    /// The watermark of a session: (last processed sequence, failed runs).
    pub async fn extraction_state(&self, session: Uuid) -> Result<Option<(i64, i64)>> {
        let row = sqlx::query(
            "SELECT last_message_sequence, failed_attempts FROM memory_extraction_state \
             WHERE session_id = ?",
        )
        .bind(session.to_string())
        .fetch_optional(&self.pool)
        .await?;
        match row {
            Some(row) => Ok(Some((
                row.try_get("last_message_sequence")?,
                row.try_get("failed_attempts")?,
            ))),
            None => Ok(None),
        }
    }

    pub async fn set_extraction_state(
        &self,
        session: Uuid,
        sequence: i64,
        failed: i64,
        now: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO memory_extraction_state \
             (session_id, last_message_sequence, failed_attempts, updated_at) VALUES (?, ?, ?, ?) \
             ON CONFLICT(session_id) DO UPDATE SET last_message_sequence = excluded.last_message_sequence, \
             failed_attempts = excluded.failed_attempts, updated_at = excluded.updated_at",
        )
        .bind(session.to_string())
        .bind(sequence)
        .bind(failed)
        .bind(now.timestamp())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// User and assistant messages of a session after `sequence`, in order:
    /// (sequence, role, text). Tool output is stored apart and not read here.
    pub async fn turns_after(
        &self,
        session: Uuid,
        sequence: i64,
    ) -> Result<Vec<(i64, String, String)>> {
        let rows = sqlx::query(
            "SELECT sequence, role, content FROM messages \
             WHERE session_id = ? AND sequence > ? AND role IN ('user', 'assistant') \
             ORDER BY sequence",
        )
        .bind(session.to_string())
        .bind(sequence)
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|row| {
                Ok((
                    row.try_get("sequence")?,
                    row.try_get("role")?,
                    row.try_get("content")?,
                ))
            })
            .collect()
    }

    /// Move a pending memory to active, recording the approval.
    pub async fn approve(&self, id: Uuid, now: DateTime<Utc>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "UPDATE memories SET status = 'active', pending_replaces = NULL, updated_at = ? \
             WHERE id = ? AND status = 'pending'",
        )
        .bind(now.timestamp())
        .bind(id.to_string())
        .execute(&mut *tx)
        .await?;
        record_event(&mut tx, id, "approved", "user", None, now).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Record the memory a pending one replaces once it is approved.
    pub async fn set_pending_replaces(&self, id: Uuid, old: Uuid) -> Result<()> {
        sqlx::query("UPDATE memories SET pending_replaces = ? WHERE id = ? AND status = 'pending'")
            .bind(old.to_string())
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .context("recording a pending replacement")?;
        Ok(())
    }

    /// The memory a pending one replaces once it is approved, if any.
    pub async fn pending_replaces(&self, id: Uuid) -> Result<Option<Uuid>> {
        let target: Option<Option<String>> =
            sqlx::query_scalar("SELECT pending_replaces FROM memories WHERE id = ?")
                .bind(id.to_string())
                .fetch_optional(&self.pool)
                .await?;
        Ok(target.flatten().and_then(|t| t.parse().ok()))
    }

    /// Pending memories of `project_id`, oldest first.
    pub async fn pending_for(&self, project_id: &ProjectId) -> Result<Vec<Memory>> {
        let rows = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories \
             WHERE status = 'pending' AND (scope = 'global' OR project_id = ?) ORDER BY created_at"
        ))
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(row_to_memory).collect()
    }

    /// The newest active episodes of a project, oldest first among them.
    pub async fn episodes_for(&self, project_id: &ProjectId, limit: i64) -> Result<Vec<Memory>> {
        let rows = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories \
             WHERE category = 'episode' AND status = 'active' AND project_id = ? \
             ORDER BY created_at DESC LIMIT ?"
        ))
        .bind(project_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        let mut episodes: Vec<Memory> = rows.iter().map(row_to_memory).collect::<Result<_>>()?;
        episodes.reverse();
        Ok(episodes)
    }

    // ── P4: path-scoped memories and sources ─────────────────────────────

    /// Attach globs to a memory. Duplicates are ignored.
    pub async fn add_paths(&self, id: Uuid, globs: &[String]) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for glob in globs {
            sqlx::query("INSERT OR IGNORE INTO memory_paths (memory_id, glob) VALUES (?, ?)")
                .bind(id.to_string())
                .bind(glob)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// The globs of one memory.
    pub async fn paths_of(&self, id: Uuid) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT glob FROM memory_paths WHERE memory_id = ? ORDER BY glob")
            .bind(id.to_string())
            .fetch_all(&self.pool)
            .await?;
        rows.iter().map(|r| Ok(r.try_get("glob")?)).collect()
    }

    /// Every (memory, glob) pair of the active memories `project_id` may use.
    pub async fn path_globs_for(&self, project_id: &ProjectId) -> Result<Vec<(Uuid, String)>> {
        let rows = sqlx::query(
            "SELECT mp.memory_id, mp.glob FROM memory_paths mp \
             JOIN memories m ON m.id = mp.memory_id \
             WHERE m.status = 'active' AND (m.scope = 'global' OR m.project_id = ?)",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|r| {
                let id: String = r.try_get("memory_id")?;
                Ok((id.parse()?, r.try_get("glob")?))
            })
            .collect()
    }

    /// Record that `sources` were folded into `consolidated` (section 13.3).
    pub async fn add_sources(&self, consolidated: Uuid, sources: &[Uuid]) -> Result<()> {
        for source in sources {
            sqlx::query(
                "INSERT OR IGNORE INTO memory_sources (memory_id, source_id) VALUES (?, ?)",
            )
            .bind(consolidated.to_string())
            .bind(source.to_string())
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    pub async fn touch(&self, id: Uuid, now: DateTime<Utc>) -> Result<()> {
        sqlx::query("UPDATE memories SET updated_at = ? WHERE id = ?")
            .bind(now.timestamp())
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Raise importance by one, up to 5, and record a `reinforced` event.
    pub async fn reinforce(&self, id: Uuid, actor: &str, now: DateTime<Utc>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "UPDATE memories SET importance = MIN(importance + 1, 5), updated_at = ? WHERE id = ?",
        )
        .bind(now.timestamp())
        .bind(id.to_string())
        .execute(&mut *tx)
        .await?;
        record_event(&mut tx, id, "reinforced", actor, None, now).await?;
        tx.commit().await?;
        Ok(())
    }

    // ── P1.9: reads ──────────────────────────────────────────────────────

    pub async fn get(&self, id: Uuid) -> Result<Option<Memory>> {
        let row = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories WHERE id = ?"
        ))
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;
        row.as_ref().map(row_to_memory).transpose()
    }

    /// Memories whose id starts with `prefix` (the short ids shown to users).
    pub async fn find_by_prefix(&self, prefix: &str) -> Result<Vec<Memory>> {
        let pattern = format!("{}%", prefix.to_lowercase());
        let rows = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories WHERE id LIKE ? ORDER BY created_at LIMIT 20"
        ))
        .bind(pattern)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(row_to_memory).collect()
    }

    /// Memories that a request for `project_id` may use: active, unexpired,
    /// and either global or in that project (§9.2). This is the only query
    /// recall starts from, so no ranking can bring in another project's memory.
    pub async fn candidates(
        &self,
        project_id: &ProjectId,
        now: DateTime<Utc>,
    ) -> Result<Vec<Memory>> {
        let rows = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories \
             WHERE status = 'active' \
             AND (expires_at IS NULL OR expires_at > ?) \
             AND (scope = 'global' OR project_id = ?) \
             ORDER BY importance DESC, updated_at DESC"
        ))
        .bind(now.timestamp())
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(row_to_memory).collect()
    }

    /// Every memory of `project_id` and the global ones, newest first. With
    /// `include_inactive` false, only active memories are returned.
    pub async fn list_for_project(
        &self,
        project_id: &ProjectId,
        include_inactive: bool,
    ) -> Result<Vec<Memory>> {
        let status_filter = if include_inactive {
            ""
        } else {
            "AND status = 'active'"
        };
        let rows = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories \
             WHERE (scope = 'global' OR project_id = ?) {status_filter} \
             ORDER BY updated_at DESC"
        ))
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(row_to_memory).collect()
    }

    /// Every memory in the database, in any scope and status, for export.
    pub async fn list_all(&self) -> Result<Vec<Memory>> {
        let rows = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories ORDER BY created_at"
        ))
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(row_to_memory).collect()
    }

    /// Events recorded for a memory, oldest first: (event, actor, time).
    pub async fn events(&self, id: Uuid) -> Result<Vec<(String, String, DateTime<Utc>)>> {
        let rows = sqlx::query(
            "SELECT event, actor, created_at FROM memory_events WHERE memory_id = ? \
             ORDER BY id",
        )
        .bind(id.to_string())
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|row| {
                Ok((
                    row.try_get::<String, _>("event")?,
                    row.try_get::<String, _>("actor")?,
                    timestamp(row.try_get("created_at")?),
                ))
            })
            .collect()
    }

    /// Known projects with their root path and number of active memories.
    pub async fn projects(&self) -> Result<Vec<(ProjectId, String, String, i64)>> {
        let rows = sqlx::query(
            "SELECT p.id, p.root_path, p.name, \
             (SELECT COUNT(*) FROM memories m WHERE m.project_id = p.id AND m.status = 'active') AS n \
             FROM memory_projects p ORDER BY p.name",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|row| {
                Ok((
                    row.try_get("id")?,
                    row.try_get("root_path")?,
                    row.try_get("name")?,
                    row.try_get("n")?,
                ))
            })
            .collect()
    }

    /// Record that these memories were put into a request.
    pub async fn record_recall(&self, ids: &[Uuid], now: DateTime<Utc>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for id in ids {
            sqlx::query(
                "UPDATE memories SET recall_count = recall_count + 1, last_recalled_at = ? \
                 WHERE id = ?",
            )
            .bind(now.timestamp())
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    // ── P1.10: search and changes ────────────────────────────────────────

    /// Keyword search with FTS5 `bm25` ranking (lower scores are better), over
    /// the same candidate set as `candidates`. `match_expr` must come from
    /// `memory::recall::fts_match_expr`, never raw user text.
    pub async fn fts_search(
        &self,
        match_expr: &str,
        project_id: &ProjectId,
        now: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<(Memory, f64)>> {
        let rows = sqlx::query(
            "SELECT m.id, m.scope, m.project_id, m.category, m.content, m.source, m.importance, \
             m.status, m.superseded_by, m.created_at, m.updated_at, m.last_recalled_at, \
             m.recall_count, m.expires_at, bm25(memories_fts) AS rank \
             FROM memories_fts JOIN memories m ON m.seq = memories_fts.rowid \
             WHERE memories_fts MATCH ? AND m.status = 'active' \
             AND (m.expires_at IS NULL OR m.expires_at > ?) \
             AND (m.scope = 'global' OR m.project_id = ?) \
             ORDER BY rank LIMIT ?",
        )
        .bind(match_expr)
        .bind(now.timestamp())
        .bind(project_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|row| Ok((row_to_memory(row)?, row.try_get::<f64, _>("rank")?)))
            .collect()
    }

    /// Replace a memory's text and hash. The FTS index follows by trigger;
    /// the vectors of the old text are deleted, so it is embedded again.
    pub async fn update_content(
        &self,
        id: Uuid,
        content: &str,
        content_hash: &str,
        actor: &str,
        now: DateTime<Utc>,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "UPDATE memories SET content = ?, content_hash = ?, updated_at = ? WHERE id = ?",
        )
        .bind(content)
        .bind(content_hash)
        .bind(now.timestamp())
        .bind(id.to_string())
        .execute(&mut *tx)
        .await?;
        sqlx::query("DELETE FROM memory_embeddings WHERE memory_id = ?")
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;
        record_event(&mut tx, id, "updated", actor, None, now).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Mark `old` as superseded by `new`. The old row is kept as history.
    pub async fn supersede(
        &self,
        old: Uuid,
        new: Uuid,
        actor: &str,
        now: DateTime<Utc>,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "UPDATE memories SET status = 'superseded', superseded_by = ?, updated_at = ? WHERE id = ?",
        )
        .bind(new.to_string())
        .bind(now.timestamp())
        .bind(old.to_string())
        .execute(&mut *tx)
        .await?;
        record_event(&mut tx, old, "superseded", actor, None, now).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Make a project memory global. The change fails if the same fact is
    /// already a global memory.
    pub async fn promote(&self, id: Uuid, actor: &str, now: DateTime<Utc>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "UPDATE memories SET scope = 'global', project_id = NULL, updated_at = ? WHERE id = ?",
        )
        .bind(now.timestamp())
        .bind(id.to_string())
        .execute(&mut *tx)
        .await
        .context("promoting memory to global")?;
        record_event(&mut tx, id, "promoted", actor, None, now).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Delete a memory. Its FTS entry goes by trigger, its vectors by cascade,
    /// and the WAL is checkpointed so old pages do not stay on disk. The event
    /// keeps the id and actor, never the content.
    pub async fn delete(&self, id: Uuid, actor: &str, now: DateTime<Utc>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        record_event(&mut tx, id, "forgotten", actor, None, now).await?;
        sqlx::query("DELETE FROM memories WHERE id = ?")
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
            .execute(&self.pool)
            .await
            .context("checkpointing the WAL after a delete")?;
        Ok(())
    }

    /// Move every memory of project `old` to project `new`, then forget `old`.
    /// `new` must already exist in memory_projects (upsert it first).
    pub async fn relink_project(&self, old: &ProjectId, new: &ProjectId) -> Result<u64> {
        // Relinking a project to itself would delete it: the final DELETE
        // cascades to its memories. Refuse before anything is written.
        if old == new {
            anyhow::bail!("that is already this project; nothing to relink");
        }
        let mut tx = self.pool.begin().await?;
        // A memory whose text the target project already holds as an active
        // or pending copy would break the unique index if moved as active. It
        // moves as archived instead, keeping its history. Everything else
        // moves unchanged.
        let moved = sqlx::query(
            "UPDATE memories SET project_id = ?1, \
             status = CASE WHEN status IN ('active', 'pending') AND content_hash IN ( \
                 SELECT content_hash FROM memories \
                 WHERE project_id = ?1 AND status IN ('active', 'pending')) \
               THEN 'archived' ELSE status END \
             WHERE project_id = ?2",
        )
        .bind(new)
        .bind(old)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        sqlx::query("DELETE FROM memory_projects WHERE id = ?")
            .bind(old)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(moved)
    }
    // ── P2.6: vectors ────────────────────────────────────────────────────

    /// Store the unit-length vector of a memory for `model`, replacing any
    /// vector already stored for that pair.
    pub async fn put_embedding(
        &self,
        id: Uuid,
        model: &str,
        vector: &[f32],
        now: DateTime<Utc>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO memory_embeddings (memory_id, model, dimensions, vector, created_at) \
             VALUES (?, ?, ?, ?, ?) \
             ON CONFLICT(memory_id, model) DO UPDATE SET \
             dimensions = excluded.dimensions, vector = excluded.vector, created_at = excluded.created_at",
        )
        .bind(id.to_string())
        .bind(model)
        .bind(i64::try_from(vector.len()).unwrap_or(i64::MAX))
        .bind(crate::memory::vector::encode(vector))
        .bind(now.timestamp())
        .execute(&self.pool)
        .await
        .context("storing memory vector")?;
        Ok(())
    }

    /// Vectors of the memories a request may use (active, unexpired, the
    /// project's or global), for one model only. A stored vector whose size
    /// does not match its recorded dimensions is skipped, never compared.
    pub async fn vectors_for(
        &self,
        project_id: &ProjectId,
        model: &str,
        now: DateTime<Utc>,
    ) -> Result<Vec<(Uuid, Vec<f32>)>> {
        let rows = sqlx::query(
            "SELECT e.memory_id, e.dimensions, e.vector FROM memory_embeddings e \
             JOIN memories m ON m.id = e.memory_id \
             WHERE e.model = ? AND m.status = 'active' \
             AND (m.expires_at IS NULL OR m.expires_at > ?) \
             AND (m.scope = 'global' OR m.project_id = ?)",
        )
        .bind(model)
        .bind(now.timestamp())
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let id: String = row.try_get("memory_id")?;
            let dims: i64 = row.try_get("dimensions")?;
            let bytes: Vec<u8> = row.try_get("vector")?;
            let Ok(dims) = usize::try_from(dims) else {
                continue;
            };
            if let (Ok(id), Ok(vector)) = (
                id.parse::<Uuid>(),
                crate::memory::vector::decode(&bytes, dims),
            ) {
                out.push((id, vector));
            }
        }
        Ok(out)
    }

    /// Active memories that have no vector for `model` yet, oldest first.
    pub async fn missing_embeddings(&self, model: &str, limit: i64) -> Result<Vec<Memory>> {
        let rows = sqlx::query(&format!(
            "SELECT {MEMORY_COLUMNS} FROM memories m \
             WHERE m.status = 'active' AND NOT EXISTS ( \
               SELECT 1 FROM memory_embeddings e WHERE e.memory_id = m.id AND e.model = ?) \
             ORDER BY m.created_at LIMIT ?"
        ))
        .bind(model)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.iter().map(row_to_memory).collect()
    }

    /// Delete vectors made by any model other than `model`. Returns the count.
    pub async fn prune_embeddings_except(&self, model: &str) -> Result<u64> {
        Ok(
            sqlx::query("DELETE FROM memory_embeddings WHERE model <> ?")
                .bind(model)
                .execute(&self.pool)
                .await?
                .rows_affected(),
        )
    }

    /// How many active memories have a vector for `model`, and how many
    /// active memories there are in all (for `memory status`).
    pub async fn embedding_counts(&self, model: &str) -> Result<(i64, i64)> {
        let indexed: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM memory_embeddings e JOIN memories m ON m.id = e.memory_id \
             WHERE e.model = ? AND m.status = 'active'",
        )
        .bind(model)
        .fetch_one(&self.pool)
        .await?;
        let active: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM memories WHERE status = 'active'")
                .fetch_one(&self.pool)
                .await?;
        Ok((indexed, active))
    }
}

/// The `scope` and `project_id` columns for a scope.
fn scope_columns(scope: &MemoryScope) -> (&'static str, Option<&str>) {
    match scope {
        MemoryScope::Global => ("global", None),
        MemoryScope::Project(id) => ("project", Some(id.as_str())),
    }
}

async fn record_event(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    memory_id: Uuid,
    event: &str,
    actor: &str,
    session_id: Option<Uuid>,
    now: DateTime<Utc>,
) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO memory_events (memory_id, event, actor, session_id, created_at) \
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(memory_id.to_string())
    .bind(event)
    .bind(actor)
    .bind(session_id.map(|s| s.to_string()))
    .bind(now.timestamp())
    .execute(&mut **tx)
    .await?;
    Ok(())
}

fn timestamp(secs: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(secs, 0).unwrap_or_else(Utc::now)
}

fn row_to_memory(row: &sqlx::sqlite::SqliteRow) -> Result<Memory> {
    let scope = match row.try_get::<Option<String>, _>("project_id")? {
        Some(project) => MemoryScope::Project(project),
        None => MemoryScope::Global,
    };
    let superseded_by: Option<String> = row.try_get("superseded_by")?;
    Ok(Memory {
        id: row.try_get::<String, _>("id")?.parse()?,
        scope,
        category: row.try_get::<String, _>("category")?.parse()?,
        content: row.try_get("content")?,
        source: row.try_get::<String, _>("source")?.parse()?,
        importance: u8::try_from(row.try_get::<i64, _>("importance")?).unwrap_or(3),
        status: row.try_get::<String, _>("status")?.parse()?,
        superseded_by: superseded_by.map(|s| s.parse()).transpose()?,
        created_at: timestamp(row.try_get("created_at")?),
        updated_at: timestamp(row.try_get("updated_at")?),
        last_recalled_at: row
            .try_get::<Option<i64>, _>("last_recalled_at")?
            .map(timestamp),
        recall_count: u32::try_from(row.try_get::<i64, _>("recall_count")?).unwrap_or(0),
        expires_at: row.try_get::<Option<i64>, _>("expires_at")?.map(timestamp),
    })
}
