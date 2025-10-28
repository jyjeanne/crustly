//! File Repository
//!
//! Database operations for file tracking.

use crate::db::models::File;
use anyhow::{Context, Result};
use sqlx::SqlitePool;

/// Repository for file operations
#[derive(Clone)]
pub struct FileRepository {
    pool: SqlitePool,
}

impl FileRepository {
    /// Create a new file repository
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Find file by ID
    pub async fn find_by_id(&self, id: &str) -> Result<Option<File>> {
        let file = sqlx::query_as::<_, File>(
            "SELECT * FROM files WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .context("Failed to find file")?;

        Ok(file)
    }

    /// Create a new file record
    pub async fn create(&self, file: &File) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO files (id, session_id, path, operation, content_hash, size_bytes, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&file.id)
        .bind(&file.session_id)
        .bind(&file.path)
        .bind(&file.operation)
        .bind(&file.content_hash)
        .bind(file.size_bytes)
        .bind(file.created_at)
        .execute(&self.pool)
        .await
        .context("Failed to create file record")?;

        tracing::debug!("Created file record: {} - {}", file.id, file.path);
        Ok(())
    }

    /// Update an existing file record
    pub async fn update(&self, file: &File) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE files
            SET path = ?, operation = ?, content_hash = ?, size_bytes = ?
            WHERE id = ?
            "#
        )
        .bind(&file.path)
        .bind(&file.operation)
        .bind(&file.content_hash)
        .bind(file.size_bytes)
        .bind(&file.id)
        .execute(&self.pool)
        .await
        .context("Failed to update file")?;

        tracing::debug!("Updated file record: {}", file.id);
        Ok(())
    }

    /// Delete a file record
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM files WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to delete file")?;

        tracing::debug!("Deleted file record: {}", id);
        Ok(())
    }

    /// List all files for a session
    pub async fn list_by_session(&self, session_id: &str) -> Result<Vec<File>> {
        let files = sqlx::query_as::<_, File>(
            "SELECT * FROM files WHERE session_id = ? ORDER BY created_at DESC"
        )
        .bind(session_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to list files")?;

        Ok(files)
    }

    /// Find files by path
    pub async fn find_by_path(&self, path: &str) -> Result<Vec<File>> {
        let files = sqlx::query_as::<_, File>(
            "SELECT * FROM files WHERE path = ? ORDER BY created_at DESC"
        )
        .bind(path)
        .fetch_all(&self.pool)
        .await
        .context("Failed to find files by path")?;

        Ok(files)
    }

    /// Find files by operation type
    pub async fn list_by_operation(
        &self,
        session_id: &str,
        operation: &str,
    ) -> Result<Vec<File>> {
        let files = sqlx::query_as::<_, File>(
            "SELECT * FROM files WHERE session_id = ? AND operation = ? ORDER BY created_at DESC"
        )
        .bind(session_id)
        .bind(operation)
        .fetch_all(&self.pool)
        .await
        .context("Failed to list files by operation")?;

        Ok(files)
    }

    /// Count files in a session
    pub async fn count_by_session(&self, session_id: &str) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM files WHERE session_id = ?"
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to count files")?;

        Ok(result.0)
    }

    /// Get total bytes read/written in a session
    pub async fn total_bytes_by_session(&self, session_id: &str) -> Result<i64> {
        let result: (Option<i64>,) = sqlx::query_as(
            "SELECT SUM(size_bytes) FROM files WHERE session_id = ?"
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to sum file bytes")?;

        Ok(result.0.unwrap_or(0))
    }

    /// Delete all file records for a session
    pub async fn delete_by_session(&self, session_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM files WHERE session_id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await
            .context("Failed to delete session files")?;

        tracing::debug!("Deleted all file records for session: {}", session_id);
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
    async fn test_file_crud() {
        let db = Database::in_memory().await.expect("Failed to create database");
        let session_repo = SessionRepository::new(db.pool().clone());
        let file_repo = FileRepository::new(db.pool().clone());

        // Create session first
        let session = Session::new("Test".to_string(), "model".to_string(), "provider".to_string());
        session_repo.create(&session).await.expect("Failed to create session");

        // Create file
        let file = File::new(session.id.clone(), "/test/file.rs".to_string(), "read".to_string());
        file_repo.create(&file).await.expect("Failed to create file");

        // Read
        let found = file_repo.find_by_id(&file.id).await.expect("Failed to find");
        assert!(found.is_some());
        assert_eq!(found.unwrap().path, "/test/file.rs");

        // Update
        let mut updated = file.clone();
        updated.operation = "write".to_string();
        file_repo.update(&updated).await.expect("Failed to update");

        let found = file_repo.find_by_id(&file.id).await.expect("Failed to find");
        assert_eq!(found.unwrap().operation, "write");

        // Delete
        file_repo.delete(&file.id).await.expect("Failed to delete");
        let found = file_repo.find_by_id(&file.id).await.expect("Failed to find");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_file_list_by_session() {
        let db = Database::in_memory().await.expect("Failed to create database");
        let session_repo = SessionRepository::new(db.pool().clone());
        let file_repo = FileRepository::new(db.pool().clone());

        let session = Session::new("Test".to_string(), "model".to_string(), "provider".to_string());
        session_repo.create(&session).await.expect("Failed to create session");

        // Create multiple files
        for i in 0..3 {
            let file = File::new(
                session.id.clone(),
                format!("/test/file{}.rs", i),
                "read".to_string(),
            );
            file_repo.create(&file).await.expect("Failed to create file");
        }

        let files = file_repo.list_by_session(&session.id).await.expect("Failed to list");
        assert_eq!(files.len(), 3);

        let count = file_repo.count_by_session(&session.id).await.expect("Failed to count");
        assert_eq!(count, 3);
    }
}
