//! Integration tests for context compaction (QS-1.2, FR-003, Contract 8).
//!
//! Run with: cargo test compaction

use crustly::db::Database;
use crustly::llm::agent::compaction::compact;
use crustly::llm::agent::context::AgentContext;
use crustly::llm::provider::types::{ContentBlock, Message, Role};
use uuid::Uuid;

/// Insert a minimal session row so FK constraints are satisfied.
async fn create_session(pool: &sqlx::SqlitePool, session_id: Uuid) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    sqlx::query(
        "INSERT INTO sessions (id, title, model, created_at, updated_at) \
         VALUES (?, 'Test', 'claude-3-sonnet', ?, ?)",
    )
    .bind(session_id.to_string())
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .unwrap();
}

fn text_message(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentBlock::Text {
            text: text.to_string(),
        }],
    }
}

/// Build a context with `n` messages.
fn build_context(session_id: Uuid, n: usize, max_tokens: usize) -> AgentContext {
    let mut ctx = AgentContext::new(session_id, max_tokens);
    for i in 0..n {
        let role = if i % 2 == 0 {
            Role::User
        } else {
            Role::Assistant
        };
        ctx.add_message(text_message(
            role,
            &format!("This is message number {} in the conversation.", i),
        ));
    }
    ctx
}

/// QS-1.2 / Contract 8.1: After compaction, last 10 turns must be verbatim.
#[tokio::test]
async fn compaction_preserves_last_10_turns() {
    let db = Database::connect_in_memory().await.expect("db");
    db.run_migrations().await.expect("migrations");

    let session_id = Uuid::new_v4();
    create_session(db.pool(), session_id).await;
    let mut ctx = build_context(session_id, 50, 200_000);

    // Capture the last 10 messages before compaction
    let before_last_10: Vec<String> = ctx.messages[40..]
        .iter()
        .flat_map(|m| m.content.iter())
        .filter_map(|b| {
            if let ContentBlock::Text { text } = b {
                Some(text.clone())
            } else {
                None
            }
        })
        .collect();

    let record = compact(&mut ctx, db.pool()).await.expect("compact");

    // Post-compaction: context should have 1 summary message + 10 preserved turns = 11
    assert_eq!(
        ctx.messages.len(),
        11,
        "expected 11 messages after compaction"
    );

    // The last 10 messages must match the original last 10 verbatim
    let after_last_10: Vec<String> = ctx.messages[1..]
        .iter()
        .flat_map(|m| m.content.iter())
        .filter_map(|b| {
            if let ContentBlock::Text { text } = b {
                Some(text.clone())
            } else {
                None
            }
        })
        .collect();

    assert_eq!(
        before_last_10, after_last_10,
        "last 10 turns must be preserved verbatim"
    );
    assert!(record.tokens_before > 0);
    assert!(record.turn_range_end == 40);
}

/// Contract 8.3: CompactionRecord must be written to DB BEFORE context is modified.
/// If DB write fails, context must remain unchanged.
///
/// We verify the atomicity property indirectly: compact() on a context with ≤10
/// messages returns Err without modifying the context.
#[tokio::test]
async fn compaction_fails_gracefully_with_insufficient_turns() {
    let db = Database::connect_in_memory().await.expect("db");
    db.run_migrations().await.expect("migrations");

    let session_id = Uuid::new_v4();
    let mut ctx = build_context(session_id, 8, 200_000); // only 8 turns
    let original_len = ctx.messages.len();
    let original_tokens = ctx.token_count;

    let result = compact(&mut ctx, db.pool()).await;
    assert!(result.is_err(), "compact must fail with <11 turns");

    // Context must be unchanged
    assert_eq!(
        ctx.messages.len(),
        original_len,
        "context must not be modified on failure"
    );
    assert_eq!(
        ctx.token_count, original_tokens,
        "token count must not be modified on failure"
    );
}

/// should_compact() threshold: context at 80% capacity triggers compaction.
#[test]
fn should_compact_fires_at_80_percent() {
    let session_id = Uuid::new_v4();
    let max_tokens = 1000;
    let mut ctx = AgentContext::new(session_id, max_tokens);

    // Manually set token_count to 79% — must NOT trigger
    ctx.token_count = 790;
    assert!(!ctx.should_compact(), "79% must not trigger compaction");

    // Set to 81% — must trigger
    ctx.token_count = 810;
    assert!(ctx.should_compact(), "81% must trigger compaction");
}

/// Compact writes exactly one CompactionRecord to the DB.
#[tokio::test]
async fn compaction_writes_one_record_to_db() {
    let db = Database::connect_in_memory().await.expect("db");
    db.run_migrations().await.expect("migrations");

    let session_id = Uuid::new_v4();
    create_session(db.pool(), session_id).await;
    let mut ctx = build_context(session_id, 30, 200_000);

    compact(&mut ctx, db.pool()).await.expect("compact");

    let repo = crustly::db::repository::CompactionRecordRepository::new(db.pool().clone());
    let records = repo.list_for_session(session_id).await.expect("list");
    assert_eq!(records.len(), 1, "expected exactly 1 CompactionRecord");
    assert_eq!(records[0].session_id, session_id);
    assert!(records[0].tokens_before > 0);
}
