//! Repository tests for long-term memory (docs/improve-memory.md §8, §9.2,
//! tasks P1.8 to P1.10). Each test runs on a fresh, migrated database.

use chrono::{DateTime, Duration, Utc};
use crustly::db::repository::memory::InsertMemory;
use crustly::db::repository::MemoryRepository;
use crustly::db::Database;
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, MemoryStatus};
use crustly::memory::project::ProjectIdentity;
use std::path::PathBuf;
use uuid::Uuid;

fn fixed_now() -> DateTime<Utc> {
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

fn new_memory(scope: MemoryScope, content: &str, status: MemoryStatus) -> InsertMemory {
    InsertMemory {
        id: Uuid::new_v4(),
        scope,
        category: MemoryCategory::Fact,
        content: content.to_string(),
        content_hash: format!("hash-{}", Uuid::new_v4()),
        source: MemorySource::User,
        importance: 3,
        status,
        session_id: None,
        expires_at: None,
        now: fixed_now(),
    }
}

/// A repository on a migrated database, with projects "a" and "b" recorded.
async fn repo_with_projects() -> (Database, MemoryRepository) {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    let repo = MemoryRepository::new(db.pool().clone());
    repo.upsert_project(&project("a"), fixed_now())
        .await
        .unwrap();
    repo.upsert_project(&project("b"), fixed_now())
        .await
        .unwrap();
    (db, repo)
}

fn project_scope(id: &str) -> MemoryScope {
    MemoryScope::Project(id.to_string())
}

#[tokio::test]
async fn saved_memory_reads_back_unchanged() {
    let (_db, repo) = repo_with_projects().await;
    let memory = new_memory(project_scope("a"), "Uses PostgreSQL", MemoryStatus::Active);
    repo.insert(&memory, "user").await.unwrap();

    let read = repo
        .get(memory.id)
        .await
        .unwrap()
        .expect("memory is stored");
    assert_eq!(read.content, "Uses PostgreSQL");
    assert_eq!(read.scope, project_scope("a"));
    assert_eq!(read.category, MemoryCategory::Fact);
    assert_eq!(read.importance, 3);
    assert_eq!(read.status, MemoryStatus::Active);
}

#[tokio::test]
async fn find_by_hash_matches_only_the_same_scope() {
    let (_db, repo) = repo_with_projects().await;
    let mut memory = new_memory(project_scope("a"), "Uses tokio", MemoryStatus::Active);
    memory.content_hash = "shared-hash".into();
    repo.insert(&memory, "user").await.unwrap();

    assert!(repo
        .find_by_hash(&project_scope("a"), "shared-hash")
        .await
        .unwrap()
        .is_some());
    assert!(repo
        .find_by_hash(&project_scope("b"), "shared-hash")
        .await
        .unwrap()
        .is_none());
    assert!(repo
        .find_by_hash(&MemoryScope::Global, "shared-hash")
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn reinforce_raises_importance_up_to_five() {
    let (_db, repo) = repo_with_projects().await;
    let memory = new_memory(MemoryScope::Global, "Prefers Tokio", MemoryStatus::Active);
    repo.insert(&memory, "user").await.unwrap();

    for _ in 0..5 {
        repo.reinforce(memory.id, "user", fixed_now())
            .await
            .unwrap();
    }
    assert_eq!(repo.get(memory.id).await.unwrap().unwrap().importance, 5);
}

#[tokio::test]
async fn candidates_hide_other_projects_and_inactive_memories() {
    let (_db, repo) = repo_with_projects().await;
    let active_a = new_memory(project_scope("a"), "Project A fact", MemoryStatus::Active);
    let in_b = new_memory(project_scope("b"), "Project B fact", MemoryStatus::Active);
    let global = new_memory(
        MemoryScope::Global,
        "Global preference",
        MemoryStatus::Active,
    );
    let superseded = new_memory(project_scope("a"), "Old A fact", MemoryStatus::Superseded);
    let archived = new_memory(
        project_scope("a"),
        "Archived A fact",
        MemoryStatus::Archived,
    );
    let pending = new_memory(project_scope("a"), "Pending A fact", MemoryStatus::Pending);
    let mut expired = new_memory(project_scope("a"), "Expired A fact", MemoryStatus::Active);
    expired.expires_at = Some(fixed_now() - Duration::days(1));
    for memory in [
        &active_a,
        &in_b,
        &global,
        &superseded,
        &archived,
        &pending,
        &expired,
    ] {
        repo.insert(memory, "user").await.unwrap();
    }

    let mut found: Vec<String> = repo
        .candidates(&"a".to_string(), fixed_now())
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.content)
        .collect();
    found.sort();
    assert_eq!(
        found,
        vec![
            "Global preference".to_string(),
            "Project A fact".to_string()
        ]
    );
}

#[tokio::test]
async fn fts_search_finds_the_current_words_only() {
    let (_db, repo) = repo_with_projects().await;
    let memory = new_memory(
        project_scope("a"),
        "Uses MySQL for storage",
        MemoryStatus::Active,
    );
    repo.insert(&memory, "user").await.unwrap();
    let project_a = "a".to_string();
    assert_eq!(
        repo.fts_search("mysql", &project_a, fixed_now(), 10)
            .await
            .unwrap()
            .len(),
        1
    );

    repo.update_content(
        memory.id,
        "Uses PostgreSQL for storage",
        "new-hash",
        "user",
        fixed_now(),
    )
    .await
    .unwrap();
    assert!(repo
        .fts_search("mysql", &project_a, fixed_now(), 10)
        .await
        .unwrap()
        .is_empty());
    assert_eq!(
        repo.fts_search("postgresql", &project_a, fixed_now(), 10)
            .await
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn fts_search_never_crosses_projects() {
    let (_db, repo) = repo_with_projects().await;
    let memory = new_memory(project_scope("b"), "Uses Redis", MemoryStatus::Active);
    repo.insert(&memory, "user").await.unwrap();
    assert!(repo
        .fts_search("redis", &"a".to_string(), fixed_now(), 10)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn deleted_memory_is_gone_from_storage_and_search() {
    let (db, repo) = repo_with_projects().await;
    let memory = new_memory(
        project_scope("a"),
        "Secret project note",
        MemoryStatus::Active,
    );
    repo.insert(&memory, "user").await.unwrap();

    repo.delete(memory.id, "user", fixed_now()).await.unwrap();
    assert!(repo.get(memory.id).await.unwrap().is_none());
    assert!(repo
        .fts_search("secret", &"a".to_string(), fixed_now(), 10)
        .await
        .unwrap()
        .is_empty());

    let forgotten: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM memory_events WHERE memory_id = ? AND event = 'forgotten'",
    )
    .bind(memory.id.to_string())
    .fetch_one(db.pool())
    .await
    .unwrap();
    assert_eq!(forgotten, 1, "the audit event survives without the content");
}

#[tokio::test]
async fn supersede_keeps_history_and_hides_the_old_memory() {
    let (_db, repo) = repo_with_projects().await;
    let old = new_memory(project_scope("a"), "Uses MySQL", MemoryStatus::Active);
    let new = new_memory(project_scope("a"), "Uses PostgreSQL", MemoryStatus::Active);
    repo.insert(&old, "user").await.unwrap();
    repo.insert(&new, "user").await.unwrap();

    repo.supersede(old.id, new.id, "user", fixed_now())
        .await
        .unwrap();
    let read = repo.get(old.id).await.unwrap().unwrap();
    assert_eq!(read.status, MemoryStatus::Superseded);
    assert_eq!(read.superseded_by, Some(new.id));

    let current: Vec<String> = repo
        .candidates(&"a".to_string(), fixed_now())
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.content)
        .collect();
    assert_eq!(current, vec!["Uses PostgreSQL".to_string()]);
}

#[tokio::test]
async fn promote_makes_a_project_memory_global() {
    let (_db, repo) = repo_with_projects().await;
    let memory = new_memory(
        project_scope("a"),
        "Prefers short answers",
        MemoryStatus::Active,
    );
    repo.insert(&memory, "user").await.unwrap();

    repo.promote(memory.id, "user", fixed_now()).await.unwrap();
    assert_eq!(
        repo.get(memory.id).await.unwrap().unwrap().scope,
        MemoryScope::Global
    );
}

#[tokio::test]
async fn relink_moves_memories_to_the_new_project() {
    let (_db, repo) = repo_with_projects().await;
    let memory = new_memory(project_scope("a"), "Uses Tokio", MemoryStatus::Active);
    repo.insert(&memory, "user").await.unwrap();
    repo.upsert_project(&project("c"), fixed_now())
        .await
        .unwrap();

    let moved = repo
        .relink_project(&"a".to_string(), &"c".to_string())
        .await
        .unwrap();
    assert_eq!(moved, 1);
    assert_eq!(
        repo.get(memory.id).await.unwrap().unwrap().scope,
        project_scope("c")
    );
}
