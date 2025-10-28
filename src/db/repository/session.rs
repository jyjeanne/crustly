//! Session Repository
//!
//! Database operations for sessions.

use crate::db::models::Session;
use anyhow::{Context, Result};
use sqlx::SqlitePool;

/// Options for listing sessions
#[derive(Debug, Clone, Default)]
pub struct SessionListOptions {
    /// Include archived sessions
    pub include_archived: bool,
    /// Maximum number of sessions to return
    pub limit: Option<usize>,
    /// Number of sessions to skip
    pub offset: usize,
}

/// Repository for session operations
#[derive(Clone)]
pub struct SessionRepository {
    pool: SqlitePool,
}

impl SessionRepository {
    /// Create a new session repository
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Find session by ID
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Session>> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to find session")?;

        Ok(session)
    }

    /// Create a new session
    pub async fn create(&self, session: &Session) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO sessions (id, title, model, provider, created_at, updated_at,
                                 total_tokens, total_cost, message_count, is_archived)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&session.id)
        .bind(&session.title)
        .bind(&session.model)
        .bind(&session.provider)
        .bind(session.created_at)
        .bind(session.updated_at)
        .bind(session.total_tokens)
        .bind(session.total_cost)
        .bind(session.message_count)
        .bind(session.is_archived)
        .execute(&self.pool)
        .await
        .context("Failed to create session")?;

        tracing::debug!("Created session: {}", session.id);
        Ok(())
    }

    /// Update an existing session
    pub async fn update(&self, session: &Session) -> Result<()> {
        let updated_at = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            UPDATE sessions
            SET title = ?, model = ?, provider = ?, updated_at = ?,
                total_tokens = ?, total_cost = ?, message_count = ?, is_archived = ?
            WHERE id = ?
            "#
        )
        .bind(&session.title)
        .bind(&session.model)
        .bind(&session.provider)
        .bind(updated_at)
        .bind(session.total_tokens)
        .bind(session.total_cost)
        .bind(session.message_count)
        .bind(session.is_archived)
        .bind(&session.id)
        .execute(&self.pool)
        .await
        .context("Failed to update session")?;

        tracing::debug!("Updated session: {}", session.id);
        Ok(())
    }

    /// Delete a session
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM sessions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to delete session")?;

        tracing::debug!("Deleted session: {}", id);
        Ok(())
    }

    /// List all sessions (most recent first)
    pub async fn list(&self, options: SessionListOptions) -> Result<Vec<Session>> {
        let mut query = String::from("SELECT * FROM sessions");

        if !options.include_archived {
            query.push_str(" WHERE is_archived = 0");
        }

        query.push_str(" ORDER BY updated_at DESC");

        if let Some(limit) = options.limit {
            query.push_str(&format!(" LIMIT {} OFFSET {}", limit, options.offset));
        }

        let sessions = sqlx::query_as::<_, Session>(&query)
            .fetch_all(&self.pool)
            .await
            .context("Failed to list sessions")?;

        Ok(sessions)
    }

    /// List non-archived sessions
    pub async fn list_active(&self) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE is_archived = 0 ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to list active sessions")?;

        Ok(sessions)
    }

    /// List archived sessions
    pub async fn list_archived(&self) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE is_archived = 1 ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to list archived sessions")?;

        Ok(sessions)
    }

    /// Archive a session
    pub async fn archive(&self, id: &str) -> Result<()> {
        let updated_at = chrono::Utc::now().timestamp();

        sqlx::query(
            "UPDATE sessions SET is_archived = 1, updated_at = ? WHERE id = ?"
        )
        .bind(updated_at)
        .bind(id)
        .execute(&self.pool)
        .await
        .context("Failed to archive session")?;

        tracing::debug!("Archived session: {}", id);
        Ok(())
    }

    /// Unarchive a session
    pub async fn unarchive(&self, id: &str) -> Result<()> {
        let updated_at = chrono::Utc::now().timestamp();

        sqlx::query(
            "UPDATE sessions SET is_archived = 0, updated_at = ? WHERE id = ?"
        )
        .bind(updated_at)
        .bind(id)
        .execute(&self.pool)
        .await
        .context("Failed to unarchive session")?;

        tracing::debug!("Unarchived session: {}", id);
        Ok(())
    }

    /// Update session statistics
    pub async fn update_stats(
        &self,
        id: &str,
        tokens: i64,
        cost: f64,
        message_count_delta: i64,
    ) -> Result<()> {
        let updated_at = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            UPDATE sessions
            SET total_tokens = total_tokens + ?,
                total_cost = total_cost + ?,
                message_count = message_count + ?,
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(tokens)
        .bind(cost)
        .bind(message_count_delta)
        .bind(updated_at)
        .bind(id)
        .execute(&self.pool)
        .await
        .context("Failed to update session stats")?;

        Ok(())
    }

    /// Count sessions
    pub async fn count(&self, archived_only: bool) -> Result<i64> {
        let query = if archived_only {
            "SELECT COUNT(*) as count FROM sessions WHERE is_archived = 1"
        } else {
            "SELECT COUNT(*) as count FROM sessions WHERE is_archived = 0"
        };

        let result: (i64,) = sqlx::query_as(query)
            .fetch_one(&self.pool)
            .await
            .context("Failed to count sessions")?;

        Ok(result.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[tokio::test]
    async fn test_session_crud() {
        let db = Database::in_memory().await.expect("Failed to create database");
        let repo = SessionRepository::new(db.pool().clone());

        // Create
        let session = Session::new(
            "Test Session".to_string(),
            "claude-sonnet-4-5".to_string(),
            "anthropic".to_string(),
        );
        repo.create(&session).await.expect("Failed to create session");

        // Read
        let found = repo.find_by_id(&session.id).await.expect("Failed to find session");
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Test Session");

        // Update
        let mut updated_session = session.clone();
        updated_session.title = "Updated Title".to_string();
        repo.update(&updated_session).await.expect("Failed to update session");

        let found = repo.find_by_id(&session.id).await.expect("Failed to find session");
        assert_eq!(found.unwrap().title, "Updated Title");

        // Delete
        repo.delete(&session.id).await.expect("Failed to delete session");
        let found = repo.find_by_id(&session.id).await.expect("Failed to find session");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_session_archive() {
        let db = Database::in_memory().await.expect("Failed to create database");
        let repo = SessionRepository::new(db.pool().clone());

        let session = Session::new("Test".to_string(), "model".to_string(), "provider".to_string());
        repo.create(&session).await.expect("Failed to create session");

        // Archive
        repo.archive(&session.id).await.expect("Failed to archive");
        let found = repo.find_by_id(&session.id).await.expect("Failed to find").unwrap();
        assert!(found.is_archived);

        // Unarchive
        repo.unarchive(&session.id).await.expect("Failed to unarchive");
        let found = repo.find_by_id(&session.id).await.expect("Failed to find").unwrap();
        assert!(!found.is_archived);
    }
}
