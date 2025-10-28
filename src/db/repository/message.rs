//! Message Repository
//!
//! Database operations for messages.

use crate::db::models::Message;
use anyhow::{Context, Result};
use sqlx::SqlitePool;

/// Repository for message operations
#[derive(Clone)]
pub struct MessageRepository {
    pool: SqlitePool,
}

impl MessageRepository {
    /// Create a new message repository
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Find message by ID
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Message>> {
        let message = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to find message")?;

        Ok(message)
    }

    /// Create a new message
    pub async fn create(&self, message: &Message) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO messages (id, session_id, role, content, created_at,
                                 input_tokens, output_tokens, cost, reasoning_tokens,
                                 cache_creation_tokens, cache_read_tokens)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&message.id)
        .bind(&message.session_id)
        .bind(&message.role)
        .bind(&message.content)
        .bind(message.created_at)
        .bind(message.input_tokens)
        .bind(message.output_tokens)
        .bind(message.cost)
        .bind(message.reasoning_tokens)
        .bind(message.cache_creation_tokens)
        .bind(message.cache_read_tokens)
        .execute(&self.pool)
        .await
        .context("Failed to create message")?;

        tracing::debug!("Created message: {} in session: {}", message.id, message.session_id);
        Ok(())
    }

    /// Update an existing message
    pub async fn update(&self, message: &Message) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE messages
            SET content = ?, input_tokens = ?, output_tokens = ?, cost = ?,
                reasoning_tokens = ?, cache_creation_tokens = ?, cache_read_tokens = ?
            WHERE id = ?
            "#
        )
        .bind(&message.content)
        .bind(message.input_tokens)
        .bind(message.output_tokens)
        .bind(message.cost)
        .bind(message.reasoning_tokens)
        .bind(message.cache_creation_tokens)
        .bind(message.cache_read_tokens)
        .bind(&message.id)
        .execute(&self.pool)
        .await
        .context("Failed to update message")?;

        tracing::debug!("Updated message: {}", message.id);
        Ok(())
    }

    /// Delete a message
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM messages WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to delete message")?;

        tracing::debug!("Deleted message: {}", id);
        Ok(())
    }

    /// List all messages for a session
    pub async fn list_by_session(&self, session_id: &str) -> Result<Vec<Message>> {
        let messages = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE session_id = ? ORDER BY created_at ASC"
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to list messages")?;

        Ok(messages)
    }

    /// Count messages in a session
    pub async fn count_by_session(&self, session_id: &str) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE session_id = ?"
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to count messages")?;

        Ok(result.0)
    }

    /// Get the last message in a session
    pub async fn get_last_message(&self, session_id: &str) -> Result<Option<Message>> {
        let message = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE session_id = ? ORDER BY created_at DESC LIMIT 1"
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to get last message")?;

        Ok(message)
    }

    /// Delete all messages in a session
    pub async fn delete_by_session(&self, session_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM messages WHERE session_id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await
            .context("Failed to delete session messages")?;

        tracing::debug!("Deleted all messages for session: {}", session_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::db::models::Session;
    use crate::db::repository::SessionRepository;

    #[tokio::test]
    async fn test_message_crud() {
        let db = Database::in_memory().await.expect("Failed to create database");
        let session_repo = SessionRepository::new(db.pool().clone());
        let message_repo = MessageRepository::new(db.pool().clone());

        // Create session first
        let session = Session::new("Test".to_string(), "model".to_string(), "provider".to_string());
        session_repo.create(&session).await.expect("Failed to create session");

        // Create message
        let message = Message::new(session.id.clone(), "user".to_string(), "Hello!".to_string());
        message_repo.create(&message).await.expect("Failed to create message");

        // Read
        let found = message_repo.find_by_id(&message.id).await.expect("Failed to find");
        assert!(found.is_some());
        assert_eq!(found.unwrap().content, "Hello!");

        // Update
        let mut updated = message.clone();
        updated.content = "Updated content".to_string();
        message_repo.update(&updated).await.expect("Failed to update");

        let found = message_repo.find_by_id(&message.id).await.expect("Failed to find");
        assert_eq!(found.unwrap().content, "Updated content");

        // Delete
        message_repo.delete(&message.id).await.expect("Failed to delete");
        let found = message_repo.find_by_id(&message.id).await.expect("Failed to find");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_message_list_by_session() {
        let db = Database::in_memory().await.expect("Failed to create database");
        let session_repo = SessionRepository::new(db.pool().clone());
        let message_repo = MessageRepository::new(db.pool().clone());

        let session = Session::new("Test".to_string(), "model".to_string(), "provider".to_string());
        session_repo.create(&session).await.expect("Failed to create session");

        // Create multiple messages
        for i in 0..3 {
            let msg = Message::new(
                session.id.clone(),
                "user".to_string(),
                format!("Message {}", i),
            );
            message_repo.create(&msg).await.expect("Failed to create message");
        }

        let messages = message_repo.list_by_session(&session.id).await.expect("Failed to list");
        assert_eq!(messages.len(), 3);

        let count = message_repo.count_by_session(&session.id).await.expect("Failed to count");
        assert_eq!(count, 3);
    }
}
