//! Long-term memory tests (docs/improve-memory.md §6.2, §18.2).
//!
//! These run against a real database built by the real migrations, so the
//! schema's constraints and triggers are what production gets.

use crustly::db::Database;
use sqlx::SqlitePool;

async fn migrated_db() -> Database {
    let db = Database::connect_in_memory().await.unwrap();
    db.run_migrations().await.unwrap();
    db
}

async fn insert_project(pool: &SqlitePool, id: &str) {
    sqlx::query(
        "INSERT INTO memory_projects (id, root_path, name, created_at, last_seen_at) \
         VALUES (?, '/work/project', 'project', 0, 0)",
    )
    .bind(id)
    .execute(pool)
    .await
    .unwrap();
}

/// Insert a fact-category memory. Returns the database error, if any, so
/// tests can assert on constraints.
async fn insert_memory(
    pool: &SqlitePool,
    id: &str,
    scope: &str,
    project_id: Option<&str>,
    content: &str,
    hash: &str,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO memories \
         (id, scope, project_id, category, content, content_hash, source, status, created_at, updated_at) \
         VALUES (?, ?, ?, 'fact', ?, ?, 'user', ?, 0, 0)",
    )
    .bind(id)
    .bind(scope)
    .bind(project_id)
    .bind(content)
    .bind(hash)
    .bind(status)
    .execute(pool)
    .await
    .map(|_| ())
}

async fn fts_hits(pool: &SqlitePool, word: &str) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM memories_fts WHERE memories_fts MATCH ?")
        .bind(word)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[tokio::test]
async fn long_term_memory_tables_exist() {
    let db = migrated_db().await;
    for table in [
        "memory_projects",
        "memories",
        "memories_fts",
        "memory_events",
    ] {
        let found: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE name = ? AND type IN ('table', 'view')",
        )
        .bind(table)
        .fetch_one(db.pool())
        .await
        .unwrap();
        assert_eq!(found, 1, "table {table} must exist after migrations");
    }
}

#[tokio::test]
async fn global_memory_must_not_have_a_project() {
    let db = migrated_db().await;
    insert_project(db.pool(), "proj1").await;

    let result = insert_memory(
        db.pool(),
        "m1",
        "global",
        Some("proj1"),
        "Prefers Tokio",
        "h1",
        "active",
    )
    .await;
    assert!(
        result.is_err(),
        "a global memory with a project_id must be rejected"
    );
}

#[tokio::test]
async fn project_memory_must_have_a_project() {
    let db = migrated_db().await;

    let result = insert_memory(
        db.pool(),
        "m1",
        "project",
        None,
        "Uses PostgreSQL",
        "h1",
        "active",
    )
    .await;
    assert!(
        result.is_err(),
        "a project memory without a project_id must be rejected"
    );
}

#[tokio::test]
async fn fts_index_follows_insert_update_and_delete() {
    let db = migrated_db().await;
    let pool = db.pool();
    insert_project(pool, "proj1").await;

    insert_memory(
        pool,
        "m1",
        "project",
        Some("proj1"),
        "Prefers tokio for async",
        "h1",
        "active",
    )
    .await
    .unwrap();
    assert_eq!(
        fts_hits(pool, "tokio").await,
        1,
        "insert must index the content"
    );

    sqlx::query("UPDATE memories SET content = 'Prefers rayon for parallel work' WHERE id = 'm1'")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        fts_hits(pool, "tokio").await,
        0,
        "update must drop the old words"
    );
    assert_eq!(
        fts_hits(pool, "rayon").await,
        1,
        "update must index the new words"
    );

    sqlx::query("DELETE FROM memories WHERE id = 'm1'")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        fts_hits(pool, "rayon").await,
        0,
        "delete must remove the index entry"
    );
}

#[tokio::test]
async fn second_active_copy_of_a_fact_is_rejected() {
    let db = migrated_db().await;
    let pool = db.pool();
    insert_project(pool, "proj1").await;

    insert_memory(
        pool,
        "m1",
        "project",
        Some("proj1"),
        "Uses PostgreSQL",
        "same-hash",
        "active",
    )
    .await
    .unwrap();
    let duplicate = insert_memory(
        pool,
        "m2",
        "project",
        Some("proj1"),
        "Uses PostgreSQL",
        "same-hash",
        "active",
    )
    .await;
    assert!(
        duplicate.is_err(),
        "a second active row with the same hash in one project must fail"
    );
}

#[tokio::test]
async fn superseded_copy_of_a_fact_is_allowed() {
    let db = migrated_db().await;
    let pool = db.pool();
    insert_project(pool, "proj1").await;

    insert_memory(
        pool,
        "m1",
        "project",
        Some("proj1"),
        "Uses MySQL",
        "same-hash",
        "active",
    )
    .await
    .unwrap();
    insert_memory(
        pool,
        "m2",
        "project",
        Some("proj1"),
        "Uses MySQL",
        "same-hash",
        "superseded",
    )
    .await
    .expect("history rows with the same hash must be allowed");
}

#[tokio::test]
async fn global_facts_are_unique_across_the_global_scope() {
    let db = migrated_db().await;
    let pool = db.pool();

    insert_memory(
        pool,
        "g1",
        "global",
        None,
        "Prefers Rust",
        "g-hash",
        "active",
    )
    .await
    .unwrap();
    let duplicate = insert_memory(
        pool,
        "g2",
        "global",
        None,
        "Prefers Rust",
        "g-hash",
        "active",
    )
    .await;
    assert!(
        duplicate.is_err(),
        "two active global copies of one fact must fail"
    );
}

#[tokio::test]
async fn secure_delete_is_on_for_every_connection() {
    let db = migrated_db().await;
    // The pool may open several connections; each one must have the pragma.
    for _ in 0..3 {
        let mut conn = db.pool().acquire().await.unwrap();
        let value: i64 = sqlx::query_scalar("PRAGMA secure_delete")
            .fetch_one(&mut *conn)
            .await
            .unwrap();
        assert_eq!(
            value, 1,
            "PRAGMA secure_delete must be ON on each connection"
        );
    }
}
