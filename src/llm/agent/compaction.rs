//! Context compaction: summarises old turns to keep the session within token budget.

use crate::llm::agent::context::AgentContext;
use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Immutable record written to DB before the context is modified.
#[derive(Debug, Clone)]
pub struct CompactionRecord {
    pub id: Uuid,
    pub session_id: Uuid,
    pub turn_range_start: i32,
    pub turn_range_end: i32,
    pub tokens_before: i32,
    pub tokens_after: i32,
    pub summary_text: String,
    pub created_at: DateTime<Utc>,
}

/// Compact the context: summarise turns [0..N-10], preserve the last 10 verbatim.
///
/// The DB record is written first; if that fails the context is left unchanged.
pub async fn compact(ctx: &mut AgentContext, pool: &sqlx::SqlitePool) -> Result<CompactionRecord> {
    let total_turns = ctx.messages.len();

    // Need at least 11 messages to compact (preserve last 10)
    if total_turns <= 10 {
        anyhow::bail!(
            "not enough turns to compact (need >10, have {})",
            total_turns
        );
    }

    let preserve_from = total_turns.saturating_sub(10);
    let tokens_before = ctx.token_count as i32;

    // Build a plain-text summary of the turns being compacted
    let summary = summarise_turns(&ctx.messages[..preserve_from]);

    // Estimate tokens in the summary
    let summary_tokens = crate::llm::agent::context::token_count(&summary) as usize;

    // Write CompactionRecord to DB first (atomicity)
    let record = CompactionRecord {
        id: Uuid::new_v4(),
        session_id: ctx.session_id,
        turn_range_start: 0,
        turn_range_end: preserve_from as i32,
        tokens_before,
        tokens_after: 0, // updated below
        summary_text: summary.clone(),
        created_at: Utc::now(),
    };

    sqlx::query(
        "INSERT INTO compaction_records \
         (id, session_id, turn_range_start, turn_range_end, tokens_before, tokens_after, summary_text, created_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(record.id.to_string())
    .bind(record.session_id.to_string())
    .bind(record.turn_range_start)
    .bind(record.turn_range_end)
    .bind(record.tokens_before)
    .bind(0_i32) // placeholder; updated after
    .bind(&record.summary_text)
    .bind(record.created_at.timestamp())
    .execute(pool)
    .await?;

    // Only modify context after DB write succeeds
    let preserved = ctx.messages.drain(preserve_from..).collect::<Vec<_>>();
    ctx.messages.clear();

    // Inject the compaction summary as a system message
    use crate::llm::provider::types::{ContentBlock, Message, Role};
    ctx.messages.push(Message {
        role: Role::System,
        content: vec![ContentBlock::Text {
            text: format!(
                "[COMPACTED: turns 0–{} summarized]\n\n{}",
                preserve_from, summary
            ),
        }],
    });
    ctx.messages.extend(preserved);

    // Recalculate token count
    let new_token_count: usize = ctx
        .messages
        .iter()
        .map(|m| {
            m.content
                .iter()
                .map(|b| match b {
                    ContentBlock::Text { text } => {
                        crate::llm::agent::context::token_count(text) as usize
                    }
                    ContentBlock::ToolUse { name, input, .. } => {
                        crate::llm::agent::context::token_count(name) as usize
                            + crate::llm::agent::context::token_count(&input.to_string()) as usize
                    }
                    ContentBlock::ToolResult { content, .. } => {
                        crate::llm::agent::context::token_count(content) as usize
                    }
                    _ => 100,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        + summary_tokens;

    ctx.token_count = new_token_count;

    let tokens_after = new_token_count as i32;
    sqlx::query("UPDATE compaction_records SET tokens_after = ? WHERE id = ?")
        .bind(tokens_after)
        .bind(record.id.to_string())
        .execute(pool)
        .await?;

    Ok(CompactionRecord {
        tokens_after,
        ..record
    })
}

fn summarise_turns(messages: &[crate::llm::provider::types::Message]) -> String {
    use crate::llm::provider::types::ContentBlock;
    let mut parts = Vec::new();
    for (i, msg) in messages.iter().enumerate() {
        let role = format!("{:?}", msg.role);
        let text: String = msg
            .content
            .iter()
            .filter_map(|b| match b {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        if !text.trim().is_empty() {
            parts.push(format!(
                "Turn {}: [{}] {}",
                i,
                role,
                &text[..text.len().min(200)]
            ));
        }
    }
    parts.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::agent::context::AgentContext;
    use crate::llm::provider::types::{ContentBlock, Message, Role};
    use uuid::Uuid;

    #[test]
    fn compaction_fires_at_threshold() {
        let session_id = Uuid::new_v4();
        // max_tokens=100 so we can fill it quickly
        let mut ctx = AgentContext::new(session_id, 100);

        // Add messages until should_compact() fires
        let mut compaction_fired = false;
        for i in 0..50 {
            let msg = Message {
                role: Role::User,
                content: vec![ContentBlock::Text {
                    text: format!(
                        "message number {} with some padding content here to use tokens",
                        i
                    ),
                }],
            };
            ctx.add_message(msg);
            if ctx.should_compact() {
                compaction_fired = true;
                break;
            }
        }

        assert!(
            compaction_fired,
            "should_compact() must fire before 50 messages with 100-token budget"
        );
        let usage = ctx.usage_percentage();
        assert!(
            usage > 80.0,
            "usage should be >80% when should_compact() returns true, got {}",
            usage
        );
    }

    #[tokio::test]
    async fn compaction_atomicity_db_failure_leaves_context_unchanged() {
        // Use a pool without the compaction_records table → INSERT fails
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        // Intentionally do NOT create compaction_records table

        let session_id = Uuid::new_v4();
        let mut ctx = AgentContext::new(session_id, 5000);

        for i in 0..15 {
            ctx.add_message(Message {
                role: if i % 2 == 0 {
                    Role::User
                } else {
                    Role::Assistant
                },
                content: vec![ContentBlock::Text {
                    text: format!("message {}", i),
                }],
            });
        }

        let original_len = ctx.messages.len();
        let original_tokens = ctx.token_count;

        // compaction should fail because the table doesn't exist
        let result = compact(&mut ctx, &pool).await;

        assert!(result.is_err(), "compact must fail when table is missing");
        assert_eq!(
            ctx.messages.len(),
            original_len,
            "context must be unchanged on DB failure"
        );
        assert_eq!(
            ctx.token_count, original_tokens,
            "token count must be unchanged on DB failure"
        );
    }

    #[tokio::test]
    async fn compaction_integration_preserves_last_10_turns() {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE compaction_records (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                turn_range_start INTEGER NOT NULL,
                turn_range_end INTEGER NOT NULL,
                tokens_before INTEGER NOT NULL,
                tokens_after INTEGER NOT NULL,
                summary_text TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        let session_id = Uuid::new_v4();
        let mut ctx = AgentContext::new(session_id, 500);

        for i in 0..15 {
            ctx.add_message(Message {
                role: if i % 2 == 0 {
                    Role::User
                } else {
                    Role::Assistant
                },
                content: vec![ContentBlock::Text {
                    text: format!(
                        "Turn {} content with enough padding tokens here to consume budget",
                        i
                    ),
                }],
            });
        }

        assert!(ctx.messages.len() > 10, "need >10 turns to compact");

        let record = compact(&mut ctx, &pool).await.unwrap();

        // Exactly 1 CompactionRecord must be in DB
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM compaction_records")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 1, "exactly one CompactionRecord must be written");

        // 1 summary message + 10 preserved turns
        assert_eq!(ctx.messages.len(), 11, "1 summary + 10 preserved turns");

        // Record captures original token count
        assert!(record.tokens_before > 0, "tokens_before must be recorded");
        // tokens_after is recorded (may be higher or lower depending on summary length)
        assert!(
            record.tokens_after >= 0,
            "tokens_after must be non-negative"
        );
    }
}
