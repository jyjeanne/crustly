//! Upgrading a database made before long-term memory (docs/improve-memory.md
//! §19, task P5.3). The "old" database gets the migrations up to 0.5.3 from
//! the same migration set this build embeds; the current migrations then
//! apply on top, and existing data must survive.
//!
//! This checks the schema upgrade on this build's own bytes. It does not check
//! databases written by a different build: sqlx checksums each migration's
//! bytes, so a build with different line endings refuses them (a known issue,
//! recorded in the spec's open questions).

use crustly::db::Database;
use std::borrow::Cow;

/// The last migration that shipped in 0.5.3.
const LAST_0_5_3: i64 = 20261019000001;

#[tokio::test]
async fn a_database_from_before_memory_upgrades_and_keeps_its_sessions() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("crustly.db");
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let pool = sqlx::SqlitePool::connect(&url).await.unwrap();

    // The migrations of the old version, as this build embeds them.
    let mut old = sqlx::migrate!("./migrations");
    let old_only: Vec<_> = old
        .migrations
        .iter()
        .filter(|m| m.version <= LAST_0_5_3)
        .cloned()
        .collect();
    old.migrations = Cow::Owned(old_only);
    old.run(&pool).await.unwrap();

    let session = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO sessions (id, created_at, updated_at) VALUES (?, 1, 1)")
        .bind(&session)
        .execute(&pool)
        .await
        .unwrap();
    pool.close().await;

    // Open it with this version: the rest of the migrations apply on top.
    let db = Database::connect(&path).await.unwrap();
    db.run_migrations().await.unwrap();

    let kept: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions WHERE id = ?")
        .bind(&session)
        .fetch_one(db.pool())
        .await
        .unwrap();
    assert_eq!(kept, 1, "the old session survives the upgrade");

    let memory_tables: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN \
         ('memories', 'memory_projects', 'memory_embeddings', 'memory_extraction_state', \
          'memory_paths', 'memory_sources')",
    )
    .fetch_one(db.pool())
    .await
    .unwrap();
    assert_eq!(memory_tables, 6, "every memory table is created");

    // The tables that were never used are kept, as decided in the spec.
    let kept_tables: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('episodic_memories', 'codebase_index')",
    )
    .fetch_one(db.pool())
    .await
    .unwrap();
    assert_eq!(kept_tables, 2);
}
