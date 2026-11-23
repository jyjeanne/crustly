//! Regression tests for the minor findings of the /code-review of Phase 1:
//! duplicates that still supersede, CRLF files, submodule identity, and the
//! recall budget counting each line's overhead.

use chrono::{DateTime, Utc};
use crustly::config::MemoryConfig;
use crustly::db::Database;
use crustly::llm::agent::context::token_count;
use crustly::memory::model::{
    Memory, MemoryCategory, MemoryScope, MemorySource, MemoryStatus, NewMemory, SaveOutcome,
};
use crustly::memory::project::identify;
use crustly::memory::recall::{select, Ranked};
use crustly::memory::transfer::remove_bullets;
use crustly::memory::MemoryStore;
use std::path::PathBuf;
use uuid::Uuid;

async fn store(dir: &std::path::Path) -> (Database, MemoryStore) {
    let db = Database::connect(&dir.join("crustly.db")).await.unwrap();
    db.run_migrations().await.unwrap();
    let store = MemoryStore::new(db.pool().clone(), MemoryConfig::default());
    (db, store)
}

fn global(content: &str, replaces: Option<Uuid>) -> NewMemory {
    NewMemory {
        content: content.to_string(),
        scope: MemoryScope::Global,
        category: MemoryCategory::Fact,
        source: MemorySource::User,
        importance: 3,
        replaces,
        session_id: None,
        expires_at: None,
    }
}

#[tokio::test]
async fn a_duplicate_still_supersedes_the_memory_it_replaces() {
    let dir = tempfile::tempdir().unwrap();
    let (_db, store) = store(dir.path()).await;
    let redis = match store.save(global("Uses Redis", None)).await.unwrap() {
        SaveOutcome::Created(id) => id,
        other => panic!("{other:?}"),
    };
    let postgres = match store.save(global("Uses PostgreSQL", None)).await.unwrap() {
        SaveOutcome::Created(id) => id,
        other => panic!("{other:?}"),
    };

    // The text is already remembered, but "Uses Redis" is out of date.
    let outcome = store
        .save(global("Uses PostgreSQL", Some(redis)))
        .await
        .unwrap();
    assert_eq!(outcome, SaveOutcome::Duplicate(postgres));
    let old = store.get(redis).await.unwrap().unwrap();
    assert_eq!(
        old.status,
        MemoryStatus::Superseded,
        "the replaced memory is not left active"
    );
    assert_eq!(old.superseded_by, Some(postgres));
}

#[test]
fn removing_a_bullet_keeps_the_crlf_endings_of_the_other_lines() {
    let text =
        "# Notes\r\n\r\n## Memories\r\n- Prefers tabs\r\n- Uses cargo fmt\r\n\r\n## Other\r\n";
    let out = remove_bullets(text, &["Prefers tabs".to_string()]);
    assert_eq!(
        out,
        "# Notes\r\n\r\n## Memories\r\n- Uses cargo fmt\r\n\r\n## Other\r\n"
    );
}

#[test]
fn a_submodule_is_its_own_project_not_its_superprojects() {
    let root = tempfile::tempdir().unwrap();
    let super_git = root.path().join("super").join(".git");
    let sub_git_dir = super_git.join("modules").join("sub");
    std::fs::create_dir_all(&sub_git_dir).unwrap();
    let sub = root.path().join("super").join("sub");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(
        sub.join(".git"),
        format!("gitdir: {}\n", sub_git_dir.display()),
    )
    .unwrap();

    let identity = identify(&sub);
    let canonical_sub = std::fs::canonicalize(&sub).unwrap();
    assert_eq!(
        identity.root, canonical_sub,
        "the submodule's own directory is its root"
    );
    assert_ne!(
        identity.root,
        std::fs::canonicalize(root.path().join("super")).unwrap()
    );
}

fn memory(content: &str) -> Memory {
    let now: DateTime<Utc> = DateTime::from_timestamp(1_800_000_000, 0).unwrap();
    Memory {
        id: Uuid::new_v4(),
        scope: MemoryScope::Global,
        category: MemoryCategory::Fact,
        content: content.to_string(),
        source: MemorySource::User,
        importance: 3,
        status: MemoryStatus::Active,
        superseded_by: None,
        created_at: now,
        updated_at: now,
        last_recalled_at: None,
        recall_count: 0,
        expires_at: None,
    }
}

#[test]
fn the_budget_counts_each_lines_overhead_not_just_its_text() {
    // Three memories of about five tokens each: the text alone would fit a
    // budget of 20, but each rendered line also costs its prefix.
    let now: DateTime<Utc> = DateTime::from_timestamp(1_800_000_000, 0).unwrap();
    let candidates: Vec<Ranked> = ["one two three four five"; 3]
        .iter()
        .map(|t| Ranked {
            memory: memory(t),
            keyword: Some(-1.0),
            similarity: None,
        })
        .collect();
    let text_cost = token_count(candidates[0].memory.content.as_str()) * 3;
    assert!(
        text_cost <= 20,
        "the test needs the text alone to fit: {text_cost}"
    );
    let chosen = select(&candidates, 20, 4, now, 0.0);
    assert!(
        chosen.len() < 3,
        "the line overhead must count: {} chosen",
        chosen.len()
    );
    let _ = PathBuf::new();
}
