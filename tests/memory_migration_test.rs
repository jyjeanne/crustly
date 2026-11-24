//! Upgrading a database made before long-term memory (docs/improve-memory.md
//! §19, task P5.3). The "old" database gets the migrations up to 0.5.3 from
//! the same migration set this build embeds; the current migrations then
//! apply on top, and existing data must survive.
//!
//! This checks the schema upgrade on this build's own bytes. Databases written
//! by a build with the other line endings are handled separately: see the
//! checksum tests below and `reconcile_line_endings` in src/db/mod.rs.

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

/// A database whose stored checksum is the other line-ending form of a
/// migration (a database written by a build with other line endings) must open,
/// and its checksum is updated to this build's value.
#[tokio::test]
async fn a_checksum_recorded_with_other_line_endings_is_accepted_and_updated() {
    use sha2::{Digest, Sha384};
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("crustly.db");
    let db = Database::connect(&path).await.unwrap();
    db.run_migrations().await.unwrap();

    let migrator = sqlx::migrate!("./migrations");
    let target = migrator
        .iter()
        .find(|m| m.version == 20251028000001)
        .expect("the initial migration exists");
    let other = if target.sql.contains("\r\n") {
        target.sql.replace("\r\n", "\n")
    } else {
        target.sql.replace('\n', "\r\n")
    };
    let other_checksum = Sha384::digest(other.as_bytes()).to_vec();
    sqlx::query("UPDATE _sqlx_migrations SET checksum = ? WHERE version = 20251028000001")
        .bind(&other_checksum)
        .execute(db.pool())
        .await
        .unwrap();
    db.pool().close().await;

    let reopened = Database::connect(&path).await.unwrap();
    reopened
        .run_migrations()
        .await
        .expect("the other line endings are accepted");
    let stored: Vec<u8> =
        sqlx::query_scalar("SELECT checksum FROM _sqlx_migrations WHERE version = 20251028000001")
            .fetch_one(reopened.pool())
            .await
            .unwrap();
    assert_eq!(
        stored,
        target.checksum.as_ref().to_vec(),
        "the stored checksum is this build's"
    );
}

/// A checksum that is not the other line-ending form is still a modified
/// migration, and is refused.
#[tokio::test]
async fn an_unrelated_checksum_mismatch_is_still_refused() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("crustly.db");
    let db = Database::connect(&path).await.unwrap();
    db.run_migrations().await.unwrap();
    sqlx::query("UPDATE _sqlx_migrations SET checksum = ? WHERE version = 20251028000001")
        .bind(vec![0u8; 48])
        .execute(db.pool())
        .await
        .unwrap();
    db.pool().close().await;

    let reopened = Database::connect(&path).await.unwrap();
    assert!(
        reopened.run_migrations().await.is_err(),
        "a real modification is refused"
    );
}
