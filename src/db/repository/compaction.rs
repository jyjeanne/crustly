//! Repository for compaction records.

use crate::db::models::CompactionRecord;
use anyhow::Result;
use chrono::DateTime;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct CompactionRecordRepository {
    pool: SqlitePool,
}

impl CompactionRecordRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, rec: &CompactionRecord) -> Result<()> {
        sqlx::query(
            "INSERT INTO compaction_records \
             (id, session_id, turn_range_start, turn_range_end, \
              tokens_before, tokens_after, summary_text, created_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(rec.id.to_string())
        .bind(rec.session_id.to_string())
        .bind(rec.turn_range_start)
        .bind(rec.turn_range_end)
        .bind(rec.tokens_before)
        .bind(rec.tokens_after)
        .bind(&rec.summary_text)
        .bind(rec.created_at.timestamp())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_for_session(&self, session_id: Uuid) -> Result<Vec<CompactionRecord>> {
        #[allow(clippy::type_complexity)]
        let rows: Vec<(String, String, i32, i32, i32, i32, String, i64)> = sqlx::query_as(
            "SELECT id, session_id, turn_range_start, turn_range_end, \
             tokens_before, tokens_after, summary_text, created_at \
             FROM compaction_records WHERE session_id = ? ORDER BY created_at",
        )
        .bind(session_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(id, sid, start, end, before, after, summary, ts)| CompactionRecord {
                    id: id.parse().unwrap_or_else(|_| Uuid::new_v4()),
                    session_id: sid.parse().unwrap_or_else(|_| Uuid::new_v4()),
                    turn_range_start: start,
                    turn_range_end: end,
                    tokens_before: before,
                    tokens_after: after,
                    summary_text: summary,
                    created_at: DateTime::from_timestamp(ts, 0).unwrap_or_else(chrono::Utc::now),
                },
            )
            .collect())
    }
}
