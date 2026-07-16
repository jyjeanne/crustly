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

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to run database migrations")?;

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
}
