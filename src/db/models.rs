//! Database Models
//!
//! Data structures representing database entities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Session model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub title: Option<String>,
    pub model: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
    pub token_count: i32,
    pub total_cost: f64,
}

/// Message model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: String,
    pub content: String,
    pub sequence: i32,
    pub created_at: DateTime<Utc>,
    pub token_count: Option<i32>,
    pub cost: Option<f64>,
}

/// File model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub session_id: Uuid,
    pub path: std::path::PathBuf,
    pub content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Attachment model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Attachment {
    pub id: Uuid,
    pub message_id: Uuid,
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub mime_type: Option<String>,
    pub path: Option<std::path::PathBuf>,
    pub size_bytes: Option<i64>,
    pub created_at: DateTime<Utc>,
}

/// Tool execution model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ToolExecution {
    pub id: Uuid,
    pub message_id: Uuid,
    pub tool_name: String,
    pub arguments: String,  // JSON
    pub result: Option<String>,  // JSON
    pub status: String,
    pub approved_at: Option<DateTime<Utc>>,
    pub executed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Session {
    /// Create a new session
    pub fn new(title: Option<String>, model: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            model,
            created_at: now,
            updated_at: now,
            archived_at: None,
            token_count: 0,
            total_cost: 0.0,
        }
    }

    /// Check if the session is archived
    pub fn is_archived(&self) -> bool {
        self.archived_at.is_some()
    }
}

impl Message {
    /// Create a new message
    pub fn new(session_id: Uuid, role: String, content: String, sequence: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            role,
            content,
            sequence,
            created_at: Utc::now(),
            token_count: None,
            cost: None,
        }
    }
}

impl File {
    /// Create a new file record
    pub fn new(session_id: Uuid, path: std::path::PathBuf, content: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            session_id,
            path,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

// Manual FromRow implementations to handle type conversions
impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Session {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Session {
            id: Uuid::parse_str(row.try_get("id")?).map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            title: row.try_get("title")?,
            model: row.try_get("model")?,
            created_at: DateTime::from_timestamp(row.try_get("created_at")?, 0)
                .ok_or_else(|| sqlx::Error::Decode("Invalid timestamp for created_at".into()))?,
            updated_at: DateTime::from_timestamp(row.try_get("updated_at")?, 0)
                .ok_or_else(|| sqlx::Error::Decode("Invalid timestamp for updated_at".into()))?,
            archived_at: row.try_get::<Option<i64>, _>("archived_at")?
                .and_then(|ts| DateTime::from_timestamp(ts, 0)),
            token_count: row.try_get("token_count")?,
            total_cost: row.try_get("total_cost")?,
        })
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Message {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(Message {
            id: Uuid::parse_str(row.try_get("id")?).map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            session_id: Uuid::parse_str(row.try_get("session_id")?).map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            role: row.try_get("role")?,
            content: row.try_get("content")?,
            sequence: row.try_get("sequence")?,
            created_at: DateTime::from_timestamp(row.try_get("created_at")?, 0)
                .ok_or_else(|| sqlx::Error::Decode("Invalid timestamp for created_at".into()))?,
            token_count: row.try_get("token_count")?,
            cost: row.try_get("cost")?,
        })
    }
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for File {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

        Ok(File {
            id: Uuid::parse_str(row.try_get("id")?).map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            session_id: Uuid::parse_str(row.try_get("session_id")?).map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            path: std::path::PathBuf::from(row.try_get::<String, _>("path")?),
            content: row.try_get("content")?,
            created_at: DateTime::from_timestamp(row.try_get("created_at")?, 0)
                .ok_or_else(|| sqlx::Error::Decode("Invalid timestamp for created_at".into()))?,
            updated_at: DateTime::from_timestamp(row.try_get("updated_at")?, 0)
                .ok_or_else(|| sqlx::Error::Decode("Invalid timestamp for updated_at".into()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_new() {
        let session = Session::new(
            Some("Test Session".to_string()),
            Some("claude-sonnet-4-5".to_string()),
        );

        assert_eq!(session.title, Some("Test Session".to_string()));
        assert_eq!(session.model, Some("claude-sonnet-4-5".to_string()));
        assert_eq!(session.token_count, 0);
        assert!(!session.is_archived());
    }

    #[test]
    fn test_message_new() {
        let session_id = Uuid::new_v4();
        let message = Message::new(
            session_id,
            "user".to_string(),
            "Hello!".to_string(),
            1,
        );

        assert_eq!(message.session_id, session_id);
        assert_eq!(message.role, "user");
        assert_eq!(message.content, "Hello!");
        assert_eq!(message.sequence, 1);
        assert!(message.token_count.is_none());
    }

    #[test]
    fn test_file_new() {
        let session_id = Uuid::new_v4();
        let path = std::path::PathBuf::from("/path/to/file.rs");
        let file = File::new(
            session_id,
            path.clone(),
            None,
        );

        assert_eq!(file.session_id, session_id);
        assert_eq!(file.path, path);
        assert!(file.content.is_none());
    }

    #[test]
    fn test_session_archived() {
        let mut session = Session::new(
            Some("Test".to_string()),
            Some("model".to_string()),
        );

        assert!(!session.is_archived());

        session.archived_at = Some(Utc::now());
        assert!(session.is_archived());
    }
}
