//! Repository for episodic memories (cross-session summaries).

use crate::llm::agent::memory::EpisodicMemory;
use crate::llm::provider::types::{ContentBlock, Message, Role};
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct EpisodicMemoryRepository {
    pool: SqlitePool,
}

impl EpisodicMemoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Persist a session summary.
    pub async fn insert(&self, mem: EpisodicMemory) -> Result<()> {
        let files_json = serde_json::to_string(&mem.files_touched)?;
        let decisions_json = serde_json::to_string(&mem.decisions)?;
        let ts = mem.created_at.timestamp();
        sqlx::query(
            "INSERT INTO episodic_memories \
             (id, session_id, summary_text, token_count, files_touched, decisions, created_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(mem.id.to_string())
        .bind(mem.session_id.to_string())
        .bind(&mem.summary_text)
        .bind(mem.token_count)
        .bind(&files_json)
        .bind(&decisions_json)
        .bind(ts)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Select the most recent memories that fit within `max_tokens`.
    pub async fn list_recent(&self, limit: u32, max_tokens: i32) -> Result<Vec<EpisodicMemory>> {
        let rows: Vec<(String, String, String, i32, String, String, i64)> = sqlx::query_as(
            "SELECT id, session_id, summary_text, token_count, files_touched, decisions, created_at \
             FROM episodic_memories \
             ORDER BY created_at DESC \
             LIMIT ?",
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        let mut remaining = max_tokens;

        for (id, session_id, summary_text, token_count, files_json, decisions_json, ts) in rows {
            if token_count > remaining {
                // Contract 6: truncate instead of skipping the oversized summary
                let max_chars = (remaining as usize) * 4;
                let truncated = if max_chars < summary_text.len() {
                    format!("{}…", &summary_text[..max_chars])
                } else {
                    summary_text.clone()
                };
                result.push(EpisodicMemory {
                    id: id.parse().unwrap_or_else(|_| Uuid::new_v4()),
                    session_id: session_id.parse().unwrap_or_else(|_| Uuid::new_v4()),
                    summary_text: truncated,
                    token_count: remaining,
                    files_touched: serde_json::from_str(&files_json).unwrap_or_default(),
                    decisions: serde_json::from_str(&decisions_json).unwrap_or_default(),
                    created_at: chrono::DateTime::from_timestamp(ts, 0)
                        .unwrap_or_else(chrono::Utc::now),
                });
                break;
            }
            remaining -= token_count;
            result.push(EpisodicMemory {
                id: id.parse().unwrap_or_else(|_| Uuid::new_v4()),
                session_id: session_id.parse().unwrap_or_else(|_| Uuid::new_v4()),
                summary_text,
                token_count,
                files_touched: serde_json::from_str(&files_json).unwrap_or_default(),
                decisions: serde_json::from_str(&decisions_json).unwrap_or_default(),
                created_at: chrono::DateTime::from_timestamp(ts, 0)
                    .unwrap_or_else(chrono::Utc::now),
            });
        }

        Ok(result)
    }

    /// Inject recent episodic memories as a system message at the front of the context.
    pub async fn inject_into_context(
        &self,
        ctx: &mut crate::llm::agent::context::AgentContext,
        max_tokens: i32,
    ) -> Result<()> {
        let memories = self.list_recent(20, max_tokens).await?;
        if memories.is_empty() {
            return Ok(());
        }

        let text = format!(
            "Prior session context (most recent first):\n{}",
            memories
                .iter()
                .map(|m| format!("• {}", m.summary_text))
                .collect::<Vec<_>>()
                .join("\n")
        );

        let total_tokens: i32 = memories.iter().map(|m| m.token_count).sum();

        ctx.messages.insert(
            0,
            Message {
                role: Role::System,
                content: vec![ContentBlock::Text { text }],
            },
        );
        ctx.token_count += total_tokens as usize;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::agent::context::AgentContext;
    use uuid::Uuid;

    async fn create_test_pool() -> sqlx::SqlitePool {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE episodic_memories (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                summary_text TEXT NOT NULL,
                token_count INTEGER NOT NULL DEFAULT 0,
                files_touched TEXT NOT NULL DEFAULT '[]',
                decisions TEXT NOT NULL DEFAULT '[]',
                created_at INTEGER NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    #[tokio::test]
    async fn episodic_memory_inject_3_memories_within_budget() {
        let pool = create_test_pool().await;
        let repo = EpisodicMemoryRepository::new(pool.clone());

        // Insert 3 episodic memories, each ~400 tokens. Use a 200-char summary
        // per memory (≈50 tokens each via BPE estimate for short text).
        let session_ids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
        for (i, sid) in session_ids.iter().enumerate() {
            let summary = format!(
                "Session {} summary: implemented feature X, touched files a.rs and b.rs. \
                 Key decision: use trait objects over generics for dynamic dispatch.",
                i
            );
            let token_count = crate::llm::agent::context::token_count(&summary) as i32;
            repo.insert(EpisodicMemory {
                id: Uuid::new_v4(),
                session_id: *sid,
                summary_text: summary,
                token_count,
                files_touched: vec!["a.rs".to_string(), "b.rs".to_string()],
                decisions: vec!["use trait objects".to_string()],
                created_at: chrono::Utc::now(),
            })
            .await
            .unwrap();
        }

        let mut ctx = AgentContext::new(Uuid::new_v4(), 200_000);
        let tokens_before = ctx.token_count;

        // Inject with a generous budget (2000 tokens)
        ctx.inject_episodic_memories(&pool, 2000).await.unwrap();

        // Must have exactly 1 prepended system message containing all 3 memories
        assert_eq!(
            ctx.messages.len(),
            1,
            "should have exactly 1 injected system message"
        );
        let injected = &ctx.messages[0].content[0];
        let text = match injected {
            crate::llm::provider::types::ContentBlock::Text { text } => text,
            _ => panic!("expected Text block"),
        };
        assert!(
            text.contains("Session 0"),
            "should include session 0 summary"
        );
        assert!(
            text.contains("Session 1"),
            "should include session 1 summary"
        );
        assert!(
            text.contains("Session 2"),
            "should include session 2 summary"
        );

        // Token count must have increased
        assert!(
            ctx.token_count > tokens_before,
            "token count must increase after injection"
        );
    }
}
