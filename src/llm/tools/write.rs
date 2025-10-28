//! Write File Tool
//!
//! Allows writing content to files on the filesystem.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use tokio::fs;

/// Write file tool
pub struct WriteTool;

#[derive(Debug, Deserialize, Serialize)]
struct WriteInput {
    /// Path to the file to write
    path: String,

    /// Content to write to the file
    content: String,

    /// Whether to create parent directories if they don't exist
    #[serde(default)]
    create_dirs: bool,
}

#[async_trait]
impl Tool for WriteTool {
    fn name(&self) -> &str {
        "write_file"
    }

    fn description(&self) -> &str {
        "Write content to a file on the filesystem. Creates the file if it doesn't exist, overwrites if it does."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to write (absolute or relative to working directory)"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write to the file"
                },
                "create_dirs": {
                    "type": "boolean",
                    "description": "Whether to create parent directories if they don't exist (default: false)",
                    "default": false
                }
            },
            "required": ["path", "content"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WriteFiles, ToolCapability::SystemModification]
    }

    fn requires_approval(&self) -> bool {
        true // Writing files requires approval
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let _: WriteInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: WriteInput = serde_json::from_value(input)?;

        // Resolve path relative to working directory
        let path = if PathBuf::from(&input.path).is_absolute() {
            PathBuf::from(&input.path)
        } else {
            context.working_directory.join(&input.path)
        };

        // Create parent directories if requested
        if input.create_dirs {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .await
                    .map_err(|e| ToolError::Io(e))?;
            }
        }

        // Check if parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Ok(ToolResult::error(format!(
                    "Parent directory does not exist: {}. Use create_dirs: true to create it.",
                    parent.display()
                )));
            }
        }

        // Write the file
        fs::write(&path, &input.content)
            .await
            .map_err(|e| ToolError::Io(e))?;

        let message = format!(
            "Successfully wrote {} bytes to {}",
            input.content.len(),
            path.display()
        );

        Ok(ToolResult::success(message)
            .with_metadata("path".to_string(), path.display().to_string())
            .with_metadata("bytes".to_string(), input.content.len().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let tool = WriteTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "path": "test.txt",
            "content": "Hello, World!"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success);

        // Verify file was written
        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "Hello, World!");
    }

    #[tokio::test]
    async fn test_write_file_with_create_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("subdir").join("test.txt");

        let tool = WriteTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "path": "subdir/test.txt",
            "content": "Nested file",
            "create_dirs": true
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success);

        // Verify file was written
        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "Nested file");
    }

    #[tokio::test]
    async fn test_write_file_missing_parent_dir() {
        let temp_dir = TempDir::new().unwrap();

        let tool = WriteTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "path": "nonexistent/test.txt",
            "content": "Should fail",
            "create_dirs": false
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_write_tool_schema() {
        let tool = WriteTool;
        assert_eq!(tool.name(), "write_file");
        assert!(tool.requires_approval());

        let capabilities = tool.capabilities();
        assert!(capabilities.contains(&ToolCapability::WriteFiles));
        assert!(capabilities.contains(&ToolCapability::SystemModification));
    }

    #[tokio::test]
    async fn test_overwrite_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Write initial content
        tokio::fs::write(&file_path, "Initial content").await.unwrap();

        let tool = WriteTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "path": "test.txt",
            "content": "New content"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success);

        // Verify file was overwritten
        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "New content");
    }
}
