//! Database Models
//!
//! Data structures representing database entities.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Session model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub model: String,
    pub provider: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub total_tokens: i64,
    pub total_cost: f64,
    pub message_count: i64,
    pub is_archived: bool,
}

/// Message model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: i64,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub cost: Option<f64>,
    pub reasoning_tokens: Option<i64>,
    pub cache_creation_tokens: Option<i64>,
    pub cache_read_tokens: Option<i64>,
}

/// File model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: String,
    pub session_id: String,
    pub path: String,
    pub operation: String,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub created_at: i64,
}

/// Attachment model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Attachment {
    pub id: String,
    pub message_id: String,
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub mime_type: Option<String>,
    pub path: Option<String>,
    pub size_bytes: Option<i64>,
    pub created_at: i64,
}

/// Tool execution model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ToolExecution {
    pub id: String,
    pub message_id: String,
    pub tool_name: String,
    pub arguments: String,  // JSON
    pub result: Option<String>,  // JSON
    pub status: String,
    pub approved_at: Option<i64>,
    pub executed_at: Option<i64>,
    pub created_at: i64,
}

impl Session {
    /// Create a new session
    pub fn new(title: String, model: String, provider: String) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            model,
            provider,
            created_at: now,
            updated_at: now,
            total_tokens: 0,
            total_cost: 0.0,
            message_count: 0,
            is_archived: false,
        }
    }
}

impl Message {
    /// Create a new message
    pub fn new(session_id: String, role: String, content: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            session_id,
            role,
            content,
            created_at: chrono::Utc::now().timestamp(),
            input_tokens: None,
            output_tokens: None,
            cost: None,
            reasoning_tokens: None,
            cache_creation_tokens: None,
            cache_read_tokens: None,
        }
    }
}

impl File {
    /// Create a new file record
    pub fn new(session_id: String, path: String, operation: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            session_id,
            path,
            operation,
            content_hash: None,
            size_bytes: None,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_new() {
        let session = Session::new(
            "Test Session".to_string(),
            "claude-sonnet-4-5".to_string(),
            "anthropic".to_string(),
        );

        assert!(!session.id.is_empty());
        assert_eq!(session.title, "Test Session");
        assert_eq!(session.model, "claude-sonnet-4-5");
        assert_eq!(session.provider, "anthropic");
        assert_eq!(session.total_tokens, 0);
        assert_eq!(session.message_count, 0);
        assert!(!session.is_archived);
    }

    #[test]
    fn test_message_new() {
        let message = Message::new(
            "session-123".to_string(),
            "user".to_string(),
            "Hello!".to_string(),
        );

        assert!(!message.id.is_empty());
        assert_eq!(message.session_id, "session-123");
        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Hello!");
        assert!(message.input_tokens.is_none());
    }

    #[test]
    fn test_file_new() {
        let file = File::new(
            "session-123".to_string(),
            "/path/to/file.rs".to_string(),
            "read".to_string(),
        );

        assert!(!file.id.is_empty());
        assert_eq!(file.session_id, "session-123");
        assert_eq!(file.path, "/path/to/file.rs");
        assert_eq!(file.operation, "read");
    }
}
