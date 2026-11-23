//! Regression tests for the four important findings of the /code-review of
//! the Phase 1 branch: relinking a project to itself, relinking when both
//! projects hold the same memory, the forget tool in Plan mode, and the
//! everything-in-the-database listing that `forget --all` relies on.

use chrono::{DateTime, Utc};
use crustly::config::MemoryConfig;
use crustly::db::repository::memory::InsertMemory;
use crustly::db::repository::MemoryRepository;
use crustly::db::Database;
use crustly::llm::tools::forget_memory::ForgetMemoryTool;
use crustly::llm::tools::{Tool, ToolExecutionContext};
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, MemoryStatus};
use crustly::memory::project::ProjectIdentity;
use crustly::memory::MemoryStore;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

fn now() -> DateTime<Utc> {
    DateTime::from_timestamp(1_800_000_000, 0).unwrap()
}

fn project(id: &str) -> ProjectIdentity {
    ProjectIdentity {
        id: id.to_string(),
        root: PathBuf::from(format!("/work/{id}")),
        name: id.to_string(),
        remote_url: None,
    }
}

async fn repo() -> (Database, MemoryRepository) {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    let repo = MemoryRepository::new(db.pool().clone());
    repo.upsert_project(&project("a"), now()).await.unwrap();
    repo.upsert_project(&project("b"), now()).await.unwrap();
    (db, repo)
}

fn memory(scope: MemoryScope, content: &str, hash: &str) -> InsertMemory {
    InsertMemory {
        id: Uuid::new_v4(),
        scope,
        category: MemoryCategory::Fact,
        content: content.to_string(),
        content_hash: hash.to_string(),
        source: MemorySource::User,
        importance: 3,
        status: MemoryStatus::Active,
        session_id: None,
        expires_at: None,
        now: now(),
    }
}

#[tokio::test]
async fn relinking_a_project_to_itself_is_refused_and_deletes_nothing() {
    let (_db, repo) = repo().await;
    let kept = memory(MemoryScope::Project("a".into()), "Uses Tokio", "h1");
    repo.insert(&kept, "user").await.unwrap();

    assert!(repo
        .relink_project(&"a".to_string(), &"a".to_string())
        .await
        .is_err());
    assert!(
        repo.get(kept.id).await.unwrap().is_some(),
        "the memory survives"
    );
    assert!(repo
        .projects()
        .await
        .unwrap()
        .iter()
        .any(|(id, ..)| id == "a"));
}

#[tokio::test]
async fn relinking_moves_a_duplicate_as_archived_and_the_rest_as_active() {
    let (_db, repo) = repo().await;
    // Both projects hold "Uses Tokio" (same hash); A also has a unique fact.
    let in_a_dup = memory(MemoryScope::Project("a".into()), "Uses Tokio", "same");
    let in_b_dup = memory(MemoryScope::Project("b".into()), "Uses Tokio", "same");
    let in_a_unique = memory(MemoryScope::Project("a".into()), "Uses sqlx", "other");
    for m in [&in_a_dup, &in_b_dup, &in_a_unique] {
        repo.insert(m, "user").await.unwrap();
    }

    let moved = repo
        .relink_project(&"a".to_string(), &"b".to_string())
        .await
        .unwrap();
    assert_eq!(
        moved, 2,
        "both of A's memories move, the duplicate as archived"
    );

    let dup = repo.get(in_a_dup.id).await.unwrap().unwrap();
    assert_eq!(dup.status, MemoryStatus::Archived);
    assert_eq!(dup.scope, MemoryScope::Project("b".into()));
    let unique = repo.get(in_a_unique.id).await.unwrap().unwrap();
    assert_eq!(unique.status, MemoryStatus::Active);
    assert_eq!(unique.scope, MemoryScope::Project("b".into()));
    assert!(repo.get(in_b_dup.id).await.unwrap().is_some());
    assert!(!repo
        .projects()
        .await
        .unwrap()
        .iter()
        .any(|(id, ..)| id == "a"));
}

#[tokio::test]
async fn the_database_listing_covers_every_project() {
    // `forget --all` deletes from this listing, so it must not stop at one project.
    let (_db, repo) = repo().await;
    repo.insert(
        &memory(MemoryScope::Project("a".into()), "In A", "h-a"),
        "user",
    )
    .await
    .unwrap();
    repo.insert(
        &memory(MemoryScope::Project("b".into()), "In B", "h-b"),
        "user",
    )
    .await
    .unwrap();
    let all: Vec<String> = repo
        .list_all()
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.content)
        .collect();
    assert!(
        all.contains(&"In A".to_string()) && all.contains(&"In B".to_string()),
        "{all:?}"
    );
}

#[tokio::test]
async fn forget_is_refused_in_plan_mode_and_deletes_nothing() {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    let store = Arc::new(MemoryStore::new(db.pool().clone(), MemoryConfig::default()));
    let repo = MemoryRepository::new(db.pool().clone());
    repo.upsert_project(&project("a"), now()).await.unwrap();
    let kept = memory(MemoryScope::Global, "Prefers tabs", "h-tabs");
    repo.insert(&kept, "user").await.unwrap();

    let tool = ForgetMemoryTool::new(store);
    let mut context = ToolExecutionContext::new(Uuid::new_v4());
    context.read_only_mode = true;
    let short = kept.id.simple().to_string()[..8].to_string();
    let result = tool
        .execute(serde_json::json!({"ids": [short]}), &context)
        .await;
    assert!(result.is_err(), "plan mode refuses the forget");
    assert!(
        repo.get(kept.id).await.unwrap().is_some(),
        "nothing was deleted"
    );
}
