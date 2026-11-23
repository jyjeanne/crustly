//! Vector storage for semantic recall (docs/improve-memory.md §6.4, §6.5;
//! tasks P2.1 and P2.6). Each test runs on a fresh, migrated database.

use chrono::{DateTime, Utc};
use crustly::db::repository::memory::InsertMemory;
use crustly::db::repository::MemoryRepository;
use crustly::db::Database;
use crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, MemoryStatus};
use crustly::memory::project::ProjectIdentity;
use std::path::PathBuf;
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

async fn setup() -> (Database, MemoryRepository) {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    let repo = MemoryRepository::new(db.pool().clone());
    repo.upsert_project(&project("a"), now()).await.unwrap();
    (db, repo)
}

async fn insert(repo: &MemoryRepository, scope: MemoryScope, content: &str) -> Uuid {
    let id = Uuid::new_v4();
    repo.insert(
        &InsertMemory {
            id,
            scope,
            category: MemoryCategory::Fact,
            content: content.to_string(),
            content_hash: format!("hash-{id}"),
            source: MemorySource::User,
            importance: 3,
            status: MemoryStatus::Active,
            session_id: None,
            expires_at: None,
            now: now(),
        },
        "user",
    )
    .await
    .unwrap();
    id
}

#[tokio::test]
async fn a_stored_vector_comes_back_for_its_model_only() {
    let (_db, repo) = setup().await;
    let id = insert(&repo, MemoryScope::Project("a".into()), "Uses Tokio").await;
    repo.put_embedding(id, "ollama:nomic-embed-text", &[0.6, 0.8], now())
        .await
        .unwrap();

    let found = repo
        .vectors_for(&"a".to_string(), "ollama:nomic-embed-text", now())
        .await
        .unwrap();
    assert_eq!(found, vec![(id, vec![0.6, 0.8])]);
    let other = repo
        .vectors_for(&"a".to_string(), "openai:other", now())
        .await
        .unwrap();
    assert!(
        other.is_empty(),
        "vectors from another model are never returned"
    );
}

#[tokio::test]
async fn vectors_of_another_projects_are_never_returned() {
    let (_db, repo) = setup().await;
    repo.upsert_project(&project("b"), now()).await.unwrap();
    let in_b = insert(&repo, MemoryScope::Project("b".into()), "Uses Redis").await;
    repo.put_embedding(in_b, "m", &[1.0], now()).await.unwrap();
    assert!(repo
        .vectors_for(&"a".to_string(), "m", now())
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn a_second_put_replaces_the_first_vector() {
    let (_db, repo) = setup().await;
    let id = insert(&repo, MemoryScope::Global, "Prefers tabs").await;
    repo.put_embedding(id, "m", &[1.0, 0.0], now())
        .await
        .unwrap();
    repo.put_embedding(id, "m", &[0.0, 1.0], now())
        .await
        .unwrap();
    let found = repo
        .vectors_for(&"a".to_string(), "m", now())
        .await
        .unwrap();
    assert_eq!(found, vec![(id, vec![0.0, 1.0])]);
}

#[tokio::test]
async fn a_vector_whose_size_does_not_match_its_record_is_skipped() {
    let (db, repo) = setup().await;
    let good = insert(&repo, MemoryScope::Global, "Good").await;
    let bad = insert(&repo, MemoryScope::Global, "Bad").await;
    repo.put_embedding(good, "m", &[1.0, 0.0], now())
        .await
        .unwrap();
    // Record three dimensions for a two-value vector: corrupt on purpose.
    sqlx::query("INSERT INTO memory_embeddings (memory_id, model, dimensions, vector, created_at) VALUES (?, 'm', 3, ?, 0)")
        .bind(bad.to_string())
        .bind(crustly::memory::vector::encode(&[1.0, 0.0]))
        .execute(db.pool())
        .await
        .unwrap();
    let found = repo
        .vectors_for(&"a".to_string(), "m", now())
        .await
        .unwrap();
    assert_eq!(
        found.len(),
        1,
        "the corrupt vector is not compared: {found:?}"
    );
    assert_eq!(found[0].0, good);
}

#[tokio::test]
async fn forgetting_a_memory_removes_its_vector() {
    let (db, repo) = setup().await;
    let id = insert(&repo, MemoryScope::Global, "Prefers tabs").await;
    repo.put_embedding(id, "m", &[1.0], now()).await.unwrap();
    repo.delete(id, "user", now()).await.unwrap();

    let left: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM memory_embeddings WHERE memory_id = ?")
            .bind(id.to_string())
            .fetch_one(db.pool())
            .await
            .unwrap();
    assert_eq!(left, 0, "the vector cascades with its memory");
}

#[tokio::test]
async fn missing_lists_active_memories_without_a_vector_for_the_model() {
    let (_db, repo) = setup().await;
    let with = insert(&repo, MemoryScope::Global, "Has a vector").await;
    let without = insert(&repo, MemoryScope::Global, "Needs one").await;
    repo.put_embedding(with, "m", &[1.0], now()).await.unwrap();

    let missing: Vec<Uuid> = repo
        .missing_embeddings("m", 10)
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.id)
        .collect();
    assert_eq!(missing, vec![without]);
    assert_eq!(
        repo.missing_embeddings("new-model", 10)
            .await
            .unwrap()
            .len(),
        2
    );
}

#[tokio::test]
async fn prune_keeps_only_the_current_model_and_counts_match() {
    let (_db, repo) = setup().await;
    let id = insert(&repo, MemoryScope::Global, "Fact").await;
    repo.put_embedding(id, "old", &[1.0], now()).await.unwrap();
    repo.put_embedding(id, "new", &[1.0], now()).await.unwrap();

    assert_eq!(repo.embedding_counts("new").await.unwrap(), (1, 1));
    assert_eq!(repo.prune_embeddings_except("new").await.unwrap(), 1);
    assert_eq!(repo.embedding_counts("old").await.unwrap(), (0, 1));
}
