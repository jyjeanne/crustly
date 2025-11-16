//! Read File Tool
//!
//! Allows reading file contents from the filesystem.

use super::error::{validate_path_safety, Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;

/// Read file tool
pub struct ReadTool;

#[derive(Debug, Deserialize, Serialize)]
struct ReadInput {
    /// Path to the file to read
    path: String,

    /// Optional: Start line (0-indexed)
    #[serde(skip_serializing_if = "Option::is_none")]
    start_line: Option<usize>,

    /// Optional: Number of lines to read
    #[serde(skip_serializing_if = "Option::is_none")]
    line_count: Option<usize>,
}

#[async_trait]
impl Tool for ReadTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Read contents of a file from the filesystem. Can optionally read specific line ranges."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to read (absolute or relative to working directory)"
                },
                "start_line": {
                    "type": "integer",
                    "description": "Optional: Starting line number (0-indexed)",
                    "minimum": 0
                },
                "line_count": {
                    "type": "integer",
                    "description": "Optional: Number of lines to read from start_line",
                    "minimum": 1
                }
            },
            "required": ["path"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ReadFiles]
    }

    fn requires_approval(&self) -> bool {
        false // Reading files is generally safe
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let _: ReadInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: ReadInput = serde_json::from_value(input)?;

        // Validate path is safe and within working directory (prevents path traversal)
        let path = match validate_path_safety(&input.path, &context.working_directory) {
            Ok(p) => p,
            Err(ToolError::PermissionDenied(msg)) => {
                return Ok(ToolResult::error(format!("Access denied: {}", msg)));
            }
            Err(ToolError::InvalidInput(msg)) => {
                return Ok(ToolResult::error(format!("Invalid path: {}", msg)));
            }
            Err(e) => return Err(e),
        };

        // Check if file exists
        if !path.exists() {
            return Ok(ToolResult::error(format!(
                "File not found: {}",
                path.display()
            )));
        }

        // Check if it's a file (not a directory)
        if !path.is_file() {
            return Ok(ToolResult::error(format!(
                "Path is not a file: {}",
                path.display()
            )));
        }

        // Read file contents
        let contents = fs::read_to_string(&path).await.map_err(ToolError::Io)?;

        // Apply line range if specified
        let output = if input.start_line.is_some() || input.line_count.is_some() {
            let lines: Vec<&str> = contents.lines().collect();
            let start = input.start_line.unwrap_or(0);
            let count = input.line_count.unwrap_or(lines.len());
            let end = (start + count).min(lines.len());

            if start >= lines.len() {
                return Ok(ToolResult::error(format!(
                    "Start line {} exceeds file length {}",
                    start,
                    lines.len()
                )));
            }

            lines[start..end].join("\n")
        } else {
            contents
        };

        let output_len = output.len();
        Ok(ToolResult::success(output)
            .with_metadata("path".to_string(), path.display().to_string())
            .with_metadata("bytes".to_string(), output_len.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_read_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_file_path = temp_dir.path().join("test.txt");
        let mut temp_file = std::fs::File::create(&temp_file_path).unwrap();
        writeln!(temp_file, "Line 1\nLine 2\nLine 3").unwrap();
        temp_file.flush().unwrap();

        let tool = ReadTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "path": temp_file_path.to_str().unwrap()
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("Line 1"));
        assert!(result.output.contains("Line 3"));
    }

    #[tokio::test]
    async fn test_read_file_line_range() {
        let temp_dir = TempDir::new().unwrap();
        let temp_file_path = temp_dir.path().join("test.txt");
        let mut temp_file = std::fs::File::create(&temp_file_path).unwrap();
        writeln!(temp_file, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5").unwrap();
        temp_file.flush().unwrap();

        let tool = ReadTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "path": temp_file_path.to_str().unwrap(),
            "start_line": 1,
            "line_count": 2
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("Line 2"));
        assert!(result.output.contains("Line 3"));
        assert!(!result.output.contains("Line 1"));
        assert!(!result.output.contains("Line 4"));
    }

    #[tokio::test]
    async fn test_read_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let tool = ReadTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "path": "nonexistent_file.txt"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("not found"));
    }

    #[test]
    fn test_read_tool_schema() {
        let tool = ReadTool;
        assert_eq!(tool.name(), "read_file");
        assert!(!tool.requires_approval());

        let schema = tool.input_schema();
        assert!(schema.is_object());
    }
}
