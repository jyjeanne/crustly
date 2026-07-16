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
/// The whole post-compaction result (summary, preserved messages, new token
/// count) is computed first and written to the DB in a single INSERT; `ctx`
/// is only mutated after that write succeeds, so a DB failure genuinely
/// leaves the context unchanged and never leaves a placeholder record behind.
pub async fn compact(ctx: &mut AgentContext, pool: &sqlx::SqlitePool) -> Result<CompactionRecord> {
    use crate::llm::provider::types::{ContentBlock, Message, Role};

    let total_turns = ctx.messages.len();

    // Need at least 11 messages to compact (preserve last 10)
    if total_turns <= 10 {
        anyhow::bail!(
            "not enough turns to compact (need >10, have {})",
            total_turns
        );
    }

    // Never split a tool_use/tool_result pair across the boundary: a
    // ToolResult message with no matching ToolUse in the preserved slice
    // would violate the provider API contract on the next request.
    let mut preserve_from = total_turns.saturating_sub(10);
    while preserve_from > 0 && message_has_tool_result(&ctx.messages[preserve_from]) {
        preserve_from -= 1;
    }

    let tokens_before = ctx.token_count as i32;

    // Build a plain-text summary of the turns being compacted
    let summary = summarise_turns(&ctx.messages[..preserve_from]);

    let summary_message = Message {
        role: Role::System,
        content: vec![ContentBlock::Text {
            text: format!(
                "[COMPACTED: turns 0–{} summarized]\n\n{}",
                preserve_from, summary
            ),
        }],
    };

    let mut new_messages = Vec::with_capacity(total_turns - preserve_from + 1);
    new_messages.push(summary_message);
    new_messages.extend(ctx.messages[preserve_from..].iter().cloned());

    let new_token_count: usize = new_messages
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
        .sum::<usize>();

    let record = CompactionRecord {
        id: Uuid::new_v4(),
        session_id: ctx.session_id,
        turn_range_start: 0,
        turn_range_end: preserve_from as i32,
        tokens_before,
        tokens_after: new_token_count as i32,
        summary_text: summary,
        created_at: Utc::now(),
    };

    // Single atomic write with the final tokens_after already known — no
    // placeholder row, no second statement that can fail after the context
    // has already been mutated.
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
    .bind(record.tokens_after)
    .bind(&record.summary_text)
    .bind(record.created_at.timestamp())
    .execute(pool)
    .await?;

    // Only mutate the live context after the DB write has succeeded.
    ctx.messages = new_messages;
    ctx.token_count = new_token_count;

    Ok(record)
}

fn message_has_tool_result(msg: &crate::llm::provider::types::Message) -> bool {
    use crate::llm::provider::types::ContentBlock;
    msg.content
        .iter()
        .any(|b| matches!(b, ContentBlock::ToolResult { .. }))
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
                crate::utils::truncate_at_char_boundary(&text, 200)
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
    fn summarise_turns_truncates_multibyte_text_without_panicking() {
        // 100 × '€' (3 bytes each) = 300 bytes; byte 200 falls mid-character,
        // which used to panic with direct byte slicing.
        let msgs = vec![Message {
            role: Role::User,
            content: vec![ContentBlock::Text {
                text: "€".repeat(100),
            }],
        }];
        let summary = summarise_turns(&msgs);
        assert!(summary.starts_with("Turn 0: [User] €"));
    }

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

    /// Regression: the naive `total_turns - 10` boundary operated on raw
    /// messages, not conversational turns. One logical tool-use turn is two
    /// separate `Message` entries (an assistant `ToolUse` message, then a
    /// user `ToolResult` message); if the boundary fell exactly on the
    /// `ToolResult` half, the preserved slice began with an orphaned tool
    /// result that has no matching tool use in the preserved history -
    /// which providers reject. The boundary must be pulled back to keep
    /// such a pair together.
    #[tokio::test]
    async fn compaction_never_splits_a_tool_use_result_pair() {
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
        let mut ctx = AgentContext::new(session_id, 5000);

        // 20 messages -> naive preserve_from = 20 - 10 = 10. Message 9 is
        // the assistant's ToolUse and message 10 is the matching
        // ToolResult, so the naive boundary would land exactly between them.
        for i in 0..20 {
            let msg = if i == 9 {
                Message {
                    role: Role::Assistant,
                    content: vec![ContentBlock::ToolUse {
                        id: "call_1".to_string(),
                        name: "read_file".to_string(),
                        input: serde_json::json!({"path": "a.rs"}),
                    }],
                }
            } else if i == 10 {
                Message {
                    role: Role::User,
                    content: vec![ContentBlock::ToolResult {
                        tool_use_id: "call_1".to_string(),
                        content: "file contents".to_string(),
                        is_error: None,
                    }],
                }
            } else {
                Message {
                    role: if i % 2 == 0 {
                        Role::User
                    } else {
                        Role::Assistant
                    },
                    content: vec![ContentBlock::Text {
                        text: format!("Turn {} content with padding to consume budget", i),
                    }],
                }
            };
            ctx.add_message(msg);
        }

        compact(&mut ctx, &pool).await.unwrap();

        // The first preserved message (index 1, after the summary) must not
        // be an orphaned ToolResult - the paired ToolUse must have been
        // pulled into the preserved set too.
        let first_preserved = &ctx.messages[1];
        let is_orphaned_tool_result = first_preserved
            .content
            .iter()
            .any(|b| matches!(b, ContentBlock::ToolResult { .. }));
        assert!(
            !is_orphaned_tool_result,
            "first preserved message must not be a ToolResult with no matching ToolUse: {:?}",
            first_preserved
        );
    }
}
