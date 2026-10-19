//! Context compaction: frees context so a session keeps fitting its window.
//!
//! Two tiers, cheapest first:
//! 1. [`elide_tool_results`] replaces the output of older tool calls with a
//!    one-line stub. Nothing is summarized and nothing is written to the DB:
//!    tool output is never persisted anyway, and the model can re-run a tool.
//! 2. [`compact_keeping`] drops older messages, records a [`CompactionRecord`]
//!    and keeps a plain-text summary of them in
//!    [`AgentContext::compacted_summary`], which rides in the system prompt.

use crate::llm::agent::context::{token_count, AgentContext};
use crate::llm::provider::types::{ContentBlock, Message, Role};
use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Messages kept verbatim by [`compact`].
pub const DEFAULT_KEEP_MESSAGES: usize = 10;

/// Prefix of a tool result replaced by [`elide_tool_results`].
const ELIDED_MARKER: &str = "[output elided]";

/// Tool results shorter than this are not worth eliding.
const ELIDE_MIN_CHARS: usize = 400;

/// Upper bound on the accumulated compaction summary, in bytes. Older summary
/// lines are dropped first, so repeated compactions cannot grow the system
/// prompt without limit.
const MAX_SUMMARY_BYTES: usize = 8_000;

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

/// Compact the context, keeping the last [`DEFAULT_KEEP_MESSAGES`] messages.
/// See [`compact_keeping`].
pub async fn compact(ctx: &mut AgentContext, pool: &sqlx::SqlitePool) -> Result<CompactionRecord> {
    compact_keeping(ctx, pool, DEFAULT_KEEP_MESSAGES).await
}

/// Compact the context, keeping roughly the last `keep` messages verbatim.
///
/// The kept slice always opens with a plain user message - the most recent
/// one at or before the cut - so the next request stays valid for every
/// provider: it starts with a user turn and never holds a tool result whose
/// tool use was dropped. When that user message sits before the cut (one
/// long request with many tool rounds), it is kept as the anchor and the tool
/// rounds between it and the cut are dropped: the model keeps its task and
/// its latest work.
///
/// The whole post-compaction result is computed first and written to the DB
/// in a single INSERT; `ctx` is only mutated after that write succeeds, so a
/// DB failure leaves the context unchanged and never leaves a placeholder
/// record behind.
pub async fn compact_keeping(
    ctx: &mut AgentContext,
    pool: &sqlx::SqlitePool,
    keep: usize,
) -> Result<CompactionRecord> {
    let plan = plan_compaction(&ctx.messages, keep.max(1)).ok_or_else(|| {
        anyhow::anyhow!(
            "nothing to compact ({} messages, keeping {})",
            ctx.messages.len(),
            keep
        )
    })?;

    let tokens_before = ctx.token_count as i32;

    let dropped: Vec<(usize, &Message)> = ctx
        .messages
        .iter()
        .enumerate()
        .filter(|(i, _)| *i < plan.anchor || (*i > plan.anchor && *i < plan.tail_from))
        .collect();
    let summary = summarise_turns(&dropped);
    let turn_range_end = dropped.last().map_or(0, |(i, _)| *i as i32 + 1);

    let mut new_messages = Vec::with_capacity(ctx.messages.len() - dropped.len());
    new_messages.push(ctx.messages[plan.anchor].clone());
    let tail_start = plan.tail_from.max(plan.anchor + 1);
    new_messages.extend(ctx.messages[tail_start..].iter().cloned());

    let mut probe = AgentContext::new(ctx.session_id, ctx.max_tokens);
    for message in &new_messages {
        probe.add_message(message.clone());
    }
    let new_token_count = probe.token_count;

    let record = CompactionRecord {
        id: Uuid::new_v4(),
        session_id: ctx.session_id,
        turn_range_start: 0,
        turn_range_end,
        tokens_before,
        tokens_after: new_token_count as i32,
        summary_text: summary.clone(),
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
    ctx.compacted_summary = Some(merge_summaries(ctx.compacted_summary.take(), &summary));

    Ok(record)
}

/// Replace the output of all but the `keep_recent` most recent tool-result
/// messages with a one-line stub, and return how many tokens that freed.
///
/// The tool-use / tool-result pairing is untouched, so the conversation stays
/// valid; the model sees that output was removed and can call the tool again.
pub fn elide_tool_results(ctx: &mut AgentContext, keep_recent: usize) -> usize {
    let result_messages: Vec<usize> = ctx
        .messages
        .iter()
        .enumerate()
        .filter(|(_, m)| message_has_tool_result(m))
        .map(|(i, _)| i)
        .collect();
    let cutoff = result_messages.len().saturating_sub(keep_recent);

    let before = ctx.token_count;
    let mut changed = false;
    for &i in &result_messages[..cutoff] {
        for block in &mut ctx.messages[i].content {
            if let ContentBlock::ToolResult { content, .. } = block {
                if content.len() < ELIDE_MIN_CHARS || content.starts_with(ELIDED_MARKER) {
                    continue;
                }
                let tokens = token_count(content);
                let preview = crate::utils::truncate_at_char_boundary(content, 160);
                *content = format!(
                    "{ELIDED_MARKER} {tokens} tokens of tool output were removed to free \
                     context; call the tool again if you still need them. It began: {preview}"
                );
                changed = true;
            }
        }
    }

    if changed {
        ctx.recount_tokens();
    }
    before.saturating_sub(ctx.token_count)
}

/// Which messages [`compact_keeping`] keeps: the user message at `anchor`,
/// then everything from `tail_from` on. Everything else is dropped.
struct CompactionPlan {
    anchor: usize,
    tail_from: usize,
}

fn plan_compaction(messages: &[Message], keep: usize) -> Option<CompactionPlan> {
    // Never start the kept tail on a tool result: its tool use would be
    // dropped, and providers reject an unmatched tool result.
    let mut tail_from = messages.len().checked_sub(keep)?;
    while tail_from > 0 && message_has_tool_result(&messages[tail_from]) {
        tail_from -= 1;
    }

    // The kept slice opens with the latest plain user message at or before
    // the cut: providers require a user turn first.
    let anchor = (0..=tail_from)
        .rev()
        .find(|&i| is_plain_user_message(&messages[i]))?;

    let dropped = anchor + (tail_from - anchor).saturating_sub(1);
    (dropped > 0).then_some(CompactionPlan { anchor, tail_from })
}

fn message_has_tool_result(msg: &Message) -> bool {
    msg.content
        .iter()
        .any(|b| matches!(b, ContentBlock::ToolResult { .. }))
}

fn is_plain_user_message(msg: &Message) -> bool {
    matches!(msg.role, Role::User) && !message_has_tool_result(msg)
}

/// One line per dropped message, with tool calls and the start of their
/// results, so the summary still says what was done and found.
fn summarise_turns(messages: &[(usize, &Message)]) -> String {
    use crate::utils::truncate_at_char_boundary as cut;

    let mut parts = Vec::new();
    for (i, msg) in messages {
        let mut pieces = Vec::new();
        let mut is_tool_result = false;
        for block in &msg.content {
            match block {
                ContentBlock::Text { text } if !text.trim().is_empty() => {
                    pieces.push(cut(text.trim(), 200).to_string());
                }
                ContentBlock::ToolUse { name, input, .. } => {
                    pieces.push(format!("called {name} {}", cut(&input.to_string(), 120)));
                }
                ContentBlock::ToolResult { content, .. } => {
                    is_tool_result = true;
                    pieces.push(cut(content.trim(), 120).to_string());
                }
                _ => {}
            }
        }
        if pieces.is_empty() {
            continue;
        }
        let label = if is_tool_result {
            "Tool result".to_string()
        } else {
            format!("{:?}", msg.role)
        };
        parts.push(format!("Turn {i}: [{label}] {}", pieces.join("; ")));
    }
    parts.join("\n")
}

/// Append `new` to an earlier summary, dropping the oldest lines once the
/// result passes [`MAX_SUMMARY_BYTES`].
fn merge_summaries(previous: Option<String>, new: &str) -> String {
    let mut merged = match previous {
        Some(previous) if !previous.is_empty() => format!("{previous}\n{new}"),
        _ => new.to_string(),
    };
    while merged.len() > MAX_SUMMARY_BYTES {
        match merged.find('\n') {
            Some(pos) => {
                merged.drain(..=pos);
            }
            None => {
                let start = merged.len() - MAX_SUMMARY_BYTES;
                let start = (start..merged.len())
                    .find(|&i| merged.is_char_boundary(i))
                    .unwrap_or(merged.len());
                merged.drain(..start);
            }
        }
    }
    merged
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
        let msg = Message {
            role: Role::User,
            content: vec![ContentBlock::Text {
                text: "€".repeat(100),
            }],
        };
        let summary = summarise_turns(&[(0, &msg)]);
        assert!(summary.starts_with("Turn 0: [User] €"));
    }

    #[test]
    fn summarise_turns_records_tool_calls_and_results() {
        let call = Message {
            role: Role::Assistant,
            content: vec![ContentBlock::ToolUse {
                id: "call_1".to_string(),
                name: "read_file".to_string(),
                input: serde_json::json!({"path": "src/main.rs"}),
            }],
        };
        let result = Message {
            role: Role::User,
            content: vec![ContentBlock::ToolResult {
                tool_use_id: "call_1".to_string(),
                content: "fn main() {}".to_string(),
                is_error: None,
            }],
        };
        let summary = summarise_turns(&[(3, &call), (4, &result)]);
        assert!(summary.contains("Turn 3: [Assistant] called read_file"));
        assert!(summary.contains("src/main.rs"));
        assert!(summary.contains("Turn 4: [Tool result] fn main() {}"));
    }

    #[test]
    fn merge_summaries_drops_the_oldest_lines_past_the_cap() {
        let old = (0..400)
            .map(|i| format!("Turn {i}: [User] some earlier text"))
            .collect::<Vec<_>>()
            .join("\n");
        let merged = merge_summaries(Some(old), "Turn 999: [User] newest");
        assert!(merged.len() <= MAX_SUMMARY_BYTES);
        assert!(merged.ends_with("Turn 999: [User] newest"));
        assert!(!merged.starts_with("Turn 0:"));
    }

    fn tool_round(i: usize, output: &str) -> [Message; 2] {
        [
            Message {
                role: Role::Assistant,
                content: vec![ContentBlock::ToolUse {
                    id: format!("call_{i}"),
                    name: "read_file".to_string(),
                    input: serde_json::json!({"path": format!("file{i}.rs")}),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentBlock::ToolResult {
                    tool_use_id: format!("call_{i}"),
                    content: output.to_string(),
                    is_error: None,
                }],
            },
        ]
    }

    /// Every tool result in `messages` answers a tool use that appears before it.
    fn tool_results_are_paired(messages: &[Message]) -> bool {
        let mut seen = std::collections::HashSet::new();
        for m in messages {
            for b in &m.content {
                match b {
                    ContentBlock::ToolUse { id, .. } => {
                        seen.insert(id.clone());
                    }
                    ContentBlock::ToolResult { tool_use_id, .. } if !seen.contains(tool_use_id) => {
                        return false;
                    }
                    _ => {}
                }
            }
        }
        true
    }

    #[test]
    fn elide_tool_results_keeps_recent_output_and_frees_tokens() {
        let mut ctx = AgentContext::new(Uuid::new_v4(), 8192);
        ctx.add_message(Message::user("read the files"));
        let big = "let x = 1;\n".repeat(200);
        for i in 0..4 {
            for m in tool_round(i, &big) {
                ctx.add_message(m);
            }
        }

        let freed = elide_tool_results(&mut ctx, 1);

        assert!(freed > 0, "eliding three large outputs must free tokens");
        let outputs: Vec<&str> = ctx
            .messages
            .iter()
            .flat_map(|m| m.content.iter())
            .filter_map(|b| match b {
                ContentBlock::ToolResult { content, .. } => Some(content.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(outputs.len(), 4, "the pairs themselves stay");
        assert!(outputs[..3].iter().all(|o| o.starts_with(ELIDED_MARKER)));
        assert_eq!(outputs[3], big, "the most recent output is kept");
        assert!(tool_results_are_paired(&ctx.messages));

        // Running it again finds nothing more to free.
        assert_eq!(elide_tool_results(&mut ctx, 1), 0);
    }

    #[test]
    fn elide_tool_results_leaves_short_output_alone() {
        let mut ctx = AgentContext::new(Uuid::new_v4(), 8192);
        ctx.add_message(Message::user("list"));
        for i in 0..3 {
            for m in tool_round(i, "a.rs\nb.rs") {
                ctx.add_message(m);
            }
        }
        assert_eq!(elide_tool_results(&mut ctx, 0), 0);
    }

    async fn pool_with_compaction_table() -> sqlx::SqlitePool {
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
        pool
    }

    /// One long request - a single user message followed by many tool
    /// rounds - is the common shape on a small local window. Compaction must
    /// keep the task (the user message) and the latest rounds, drop the
    /// rounds in between, and leave a request every provider accepts.
    #[tokio::test]
    async fn compaction_of_one_long_request_keeps_the_task_and_latest_rounds() {
        let pool = pool_with_compaction_table().await;
        let mut ctx = AgentContext::new(Uuid::new_v4(), 8192);
        ctx.add_message(Message::user("fix the failing test"));
        for i in 0..8 {
            for m in tool_round(i, "some output") {
                ctx.add_message(m);
            }
        }

        let record = compact_keeping(&mut ctx, &pool, 4).await.unwrap();

        assert_eq!(ctx.messages.len(), 5, "task + the last two tool rounds");
        assert!(is_plain_user_message(&ctx.messages[0]));
        assert!(matches!(
            &ctx.messages[0].content[0],
            ContentBlock::Text { text } if text == "fix the failing test"
        ));
        assert!(tool_results_are_paired(&ctx.messages));
        assert!(record.tokens_after < record.tokens_before);

        let summary = ctx.compacted_summary.as_deref().unwrap();
        assert!(summary.contains("called read_file"));
        let prompt = ctx.effective_system_prompt().unwrap();
        assert!(prompt.contains("Earlier in this conversation (compacted)"));
        assert!(prompt.contains(summary));
    }

    #[tokio::test]
    async fn compaction_refuses_when_nothing_would_be_dropped() {
        let pool = pool_with_compaction_table().await;
        let mut ctx = AgentContext::new(Uuid::new_v4(), 8192);
        ctx.add_message(Message::user("hello"));
        ctx.add_message(Message::assistant("hi"));

        assert!(compact_keeping(&mut ctx, &pool, 10).await.is_err());
        assert_eq!(ctx.messages.len(), 2);
        assert!(ctx.compacted_summary.is_none());
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

        // The cut falls on an assistant message, so the user message just
        // before it is kept as the anchor: anchor + the last 10 messages.
        assert_eq!(ctx.messages.len(), 11, "anchor user message + 10 preserved");
        assert!(is_plain_user_message(&ctx.messages[0]));
        // The summary lives in the system prompt, not in the message list.
        assert!(ctx.messages.iter().all(|m| !matches!(m.role, Role::System)));
        assert!(ctx.compacted_summary.is_some());

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

        // The kept slice opens with a plain user message, and every kept
        // ToolResult still has its ToolUse - the pair was not split.
        assert!(
            is_plain_user_message(&ctx.messages[0]),
            "kept slice must open with a user message: {:?}",
            ctx.messages[0]
        );
        assert!(
            tool_results_are_paired(&ctx.messages),
            "a ToolResult lost its ToolUse: {:?}",
            ctx.messages
        );
    }
}
