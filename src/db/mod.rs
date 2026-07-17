//! Database Layer
//!
//! Provides database connection management, models, and repositories.

pub mod models;
pub mod repository;
pub mod retry;

pub use models::*;
pub use repository::*;
pub use retry::{retry_db_anyhow, retry_db_operation, retry_db_sqlx, DbRetryConfig};

use anyhow::{Context, Result};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

/// Type alias for database pool
pub type Pool = SqlitePool;

/// Database connection manager
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Connect to a SQLite database file.
    ///
    /// A leading `~` is expanded to the home directory. The default config ships
    /// `path = "~/.crustly/crustly.db"`, and without expansion that was taken
    /// literally: Crustly created a directory *named* `~` inside whatever folder
    /// it was launched from and put the database there. The session then lived
    /// somewhere nobody would think to look, `ls` reported a stray `~/` in the
    /// workspace, and deleting `.crustly/crustly.db` did not reset anything.
    pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self> {
        let raw = path.as_ref();
        let expanded = shellexpand::tilde(&raw.to_string_lossy()).into_owned();
        let expanded = std::path::PathBuf::from(expanded);
        if expanded != raw {
            tracing::debug!("Expanded database path {:?} -> {:?}", raw, expanded);
        }
        let path = expanded.as_path();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                tracing::debug!("Creating database directory: {:?}", parent);
                std::fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create database directory: {:?}", parent)
                })?;
            }
        }

        let path_str = path.to_string_lossy().into_owned();
        let url = format!("sqlite://{}?mode=rwc", path_str);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(10))
            .after_connect(|conn, _meta| {
                Box::pin(async move {
                    // WAL mode: enables concurrent reads during writes (T018)
                    sqlx::query("PRAGMA journal_mode=WAL")
                        .execute(&mut *conn)
                        .await?;
                    sqlx::query("PRAGMA busy_timeout = 5000")
                        .execute(&mut *conn)
                        .await?;
                    // SQLite disables foreign-key enforcement per-connection
                    // by default. Every migration declares
                    // `ON DELETE CASCADE` between sessions and their
                    // messages/plans/plan_tasks/files/episodic_memories/
                    // compaction_records, but without this pragma none of
                    // it ever fired: deleting a session left every child
                    // row permanently orphaned, and a message write racing
                    // a session delete could succeed with a session_id
                    // pointing at nothing, since there was no constraint to
                    // reject it.
                    sqlx::query("PRAGMA foreign_keys = ON")
                        .execute(&mut *conn)
                        .await?;
                    Ok(())
                })
            })
            .connect(&url)
            .await
            .context("Failed to connect to database")?;

        tracing::info!("Connected to database: {} (busy_timeout: 5s)", path_str);
        Ok(Self { pool })
    }

    /// Connect to an in-memory database (for testing)
    pub async fn connect_in_memory() -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .after_connect(|conn, _meta| {
                Box::pin(async move {
                    // Match connect()'s enforcement so tests exercise the
                    // same FK behavior production runs under.
                    sqlx::query("PRAGMA foreign_keys = ON")
                        .execute(&mut *conn)
                        .await?;
                    Ok(())
                })
            })
            .connect("sqlite::memory:")
            .await
            .context("Failed to connect to in-memory database")?;

        tracing::debug!("Connected to in-memory database");
        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Check if the database connection is still valid
    pub fn is_connected(&self) -> bool {
        !self.pool.is_closed()
    }

    /// Run database migrations.
    ///
    /// Runs with foreign-key enforcement explicitly turned off on the
    /// connection it uses, then restores it before returning that
    /// connection to the pool.
    ///
    /// `connect()`'s `after_connect` hook enables `PRAGMA foreign_keys = ON`
    /// on every connection, including whichever one this pulls from the
    /// pool. That is wrong for a migration run specifically:
    /// `20251028000002_modernize_schema.sql`'s "Sessions Table Updates"
    /// section does `DROP TABLE sessions` while the *old* `messages`/`files`
    /// tables (from `20251028000001_initial_schema.sql`) still declare
    /// `FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE`
    /// and are not dropped/rebuilt until later in the same file. With FK
    /// enforcement active, SQLite treats `DROP TABLE` on a table other rows
    /// reference as an implicit `DELETE FROM` of every one of its rows
    /// first - which fires the cascade and silently destroys every row in
    /// the not-yet-migrated `messages`/`files` tables before the
    /// migration's own `INSERT INTO messages_new ... SELECT ... FROM
    /// messages` ever runs, leaving `messages_new`/`files_new` empty. Any
    /// database that has not yet applied that migration (e.g. a long-
    /// dormant install upgrading straight to a version with FK enforcement
    /// enabled) would have every message and tracked file permanently
    /// erased, with the migration itself reporting success.
    pub async fn run_migrations(&self) -> Result<()> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .context("Failed to acquire a connection to run migrations")?;

        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&mut *conn)
            .await
            .context("Failed to disable foreign keys for migration")?;

        let migration_result = sqlx::migrate!("./migrations").run(&mut conn).await;

        // Restore FK enforcement on this connection before returning it to
        // the pool, regardless of migration outcome, so it matches the
        // invariant every other pooled connection has via `after_connect`.
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&mut *conn)
            .await
            .context("Failed to re-enable foreign keys after migration")?;

        migration_result.context("Failed to run database migrations")?;

        tracing::info!("Database migrations completed");
        Ok(())
    }

    /// Close the database connection
    pub async fn close(self) -> Result<()> {
        self.pool.close().await;
        tracing::info!("Database connection closed");
        Ok(())
    }
}

/// Extension trait for SqlitePool to add convenience methods
#[allow(async_fn_in_trait)]
pub trait PoolExt {
    /// Connect to a database file
    async fn connect_file<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized;

    /// Connect to an in-memory database
    async fn connect_in_memory() -> Result<Self>
    where
        Self: Sized;

    /// Check if the pool is connected
    fn is_connected(&self) -> bool;
}

impl PoolExt for SqlitePool {
    async fn connect_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Database::connect(path).await?;
        Ok(db.pool)
    }

    async fn connect_in_memory() -> Result<Self> {
        let db = Database::connect_in_memory().await?;
        Ok(db.pool)
    }

    fn is_connected(&self) -> bool {
        !self.is_closed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect_in_memory() {
        let db = Database::connect_in_memory().await.unwrap();
        assert!(db.is_connected());
    }

    #[tokio::test]
    async fn test_pool_connect_in_memory() {
        let pool = Pool::connect_in_memory().await.unwrap();
        assert!(pool.is_connected());
    }

    /// Regression: the default config ships `path = "~/.crustly/crustly.db"`.
    /// Unexpanded, `create_dir_all` made a directory literally named `~` inside
    /// the current working directory and opened the database there - so the
    /// session was invisible, a stray `~/` polluted the user's workspace, and
    /// clearing `.crustly/` did not reset anything.
    #[tokio::test]
    async fn tilde_in_the_database_path_is_expanded_to_home() {
        let home = dirs::home_dir().expect("home dir");
        let unique = format!("crustly-tilde-test-{}", uuid::Uuid::new_v4());
        let rel = format!("~/{unique}/test.db");

        let db = Database::connect(&rel).await.expect("connects");
        assert!(db.is_connected());

        // The database must land under $HOME, and crucially NOT in a directory
        // literally named `~` inside the current working directory.
        let expected_dir = home.join(&unique);
        let literal_tilde_dir = std::path::Path::new("~").join(&unique);

        assert!(
            !literal_tilde_dir.exists(),
            "the tilde was not expanded: a directory literally named `~` was created \
             in the cwd at {literal_tilde_dir:?}"
        );
        assert!(
            expected_dir.is_dir(),
            "expected the database under home at {expected_dir:?}"
        );

        drop(db);
        let _ = std::fs::remove_dir_all(&expected_dir);
    }

    /// Regression: SQLite disables foreign-key enforcement per-connection
    /// by default, and nothing turned it on. Every migration declares
    /// `ON DELETE CASCADE` between `sessions` and its child tables, but
    /// without `PRAGMA foreign_keys = ON` none of it ever fired - a
    /// `messages` row could reference a `session_id` that does not exist,
    /// and deleting a session left every child row orphaned instead of
    /// cascading. This checks the constraint is actually live.
    #[tokio::test]
    async fn foreign_keys_are_enforced() {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();

        let bogus_session_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            "INSERT INTO messages (id, session_id, role, content, sequence, created_at) \
             VALUES (?, ?, 'user', 'hello', 1, ?)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&bogus_session_id)
        .bind(now)
        .execute(db.pool())
        .await;

        assert!(
            result.is_err(),
            "inserting a message for a session_id that does not exist must fail \
             with foreign keys enforced"
        );
    }

    /// Regression companion: with the constraint live, deleting a session
    /// must actually cascade-delete its messages (the behavior every
    /// migration's `ON DELETE CASCADE` already declared but which never
    /// fired before `PRAGMA foreign_keys = ON` was added).
    #[tokio::test]
    async fn deleting_a_session_cascades_to_its_messages() {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();

        let session_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        sqlx::query("INSERT INTO sessions (id, created_at, updated_at) VALUES (?, ?, ?)")
            .bind(&session_id)
            .bind(now)
            .bind(now)
            .execute(db.pool())
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO messages (id, session_id, role, content, sequence, created_at) \
             VALUES (?, ?, 'user', 'hello', 1, ?)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&session_id)
        .bind(now)
        .execute(db.pool())
        .await
        .unwrap();

        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(&session_id)
            .execute(db.pool())
            .await
            .unwrap();

        let remaining: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM messages WHERE session_id = ?")
                .bind(&session_id)
                .fetch_one(db.pool())
                .await
                .unwrap();

        assert_eq!(
            remaining, 0,
            "deleting a session must cascade-delete its messages, not orphan them"
        );
    }

    /// Regression: turning on `PRAGMA foreign_keys` made
    /// `20251028000002_modernize_schema.sql` destructive for a database
    /// that hasn't yet applied it. That migration's "Sessions Table
    /// Updates" section does `DROP TABLE sessions` while the *old*
    /// `messages`/`files` tables (from `20251028000001_initial_schema.sql`)
    /// still carry `FOREIGN KEY (session_id) REFERENCES sessions(id) ON
    /// DELETE CASCADE` and aren't dropped/rebuilt until later in the same
    /// file. With FK enforcement active, SQLite treats `DROP TABLE` on a
    /// referenced table as an implicit delete of every one of its rows
    /// first - cascading and destroying every row in the not-yet-migrated
    /// `messages` table before the migration's own `INSERT INTO
    /// messages_new ... SELECT ... FROM messages` ever runs.
    ///
    /// This reproduces the real upgrade path faithfully: a database that
    /// has applied only migration 1 (via a standalone runtime `Migrator`
    /// pointed at a directory containing just that one file, so
    /// `_sqlx_migrations` gets a real, correctly-checksummed row for it),
    /// with real user data inserted using migration 1's schema, then
    /// upgraded via the actual embedded `run_migrations()` used in
    /// production.
    #[tokio::test]
    async fn migrating_from_pre_modernization_schema_preserves_existing_messages() {
        let db = Database::connect_in_memory().await.unwrap();

        // Apply *only* migration 1, via a standalone directory containing
        // just that file, so `_sqlx_migrations` ends up with a real row
        // for it (correct version + checksum) exactly as sqlx would
        // compute for any real database frozen at this schema version.
        let migration_1_dir = tempfile::tempdir().unwrap();
        let migration_1_sql = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/migrations/20251028000001_initial_schema.sql"
        ))
        .unwrap();
        std::fs::write(
            migration_1_dir
                .path()
                .join("20251028000001_initial_schema.sql"),
            migration_1_sql,
        )
        .unwrap();

        sqlx::migrate::Migrator::new(migration_1_dir.path())
            .await
            .expect("load migration 1 as a standalone source")
            .run(db.pool())
            .await
            .expect("apply migration 1 only");

        // Insert real user data using migration 1's schema, exactly as an
        // existing installation frozen at this version would have.
        let session_id = uuid::Uuid::new_v4().to_string();
        let message_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        sqlx::query(
            "INSERT INTO sessions (id, title, model, provider, created_at, updated_at) \
             VALUES (?, 'Test', 'test-model', 'test-provider', ?, ?)",
        )
        .bind(&session_id)
        .bind(now)
        .bind(now)
        .execute(db.pool())
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO messages (id, session_id, role, content, created_at) \
             VALUES (?, ?, 'user', 'irreplaceable chat history', ?)",
        )
        .bind(&message_id)
        .bind(&session_id)
        .bind(now)
        .execute(db.pool())
        .await
        .unwrap();

        // Upgrade via the real, fully embedded migrator - this is exactly
        // what production does on every startup.
        db.run_migrations().await.expect("upgrade migrations");

        let surviving_content: Option<String> =
            sqlx::query_scalar("SELECT content FROM messages WHERE id = ?")
                .bind(&message_id)
                .fetch_optional(db.pool())
                .await
                .unwrap();

        assert_eq!(
            surviving_content.as_deref(),
            Some("irreplaceable chat history"),
            "upgrading from the pre-modernization schema must not lose existing messages"
        );
    }
}
