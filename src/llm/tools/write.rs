//! Write File Tool
//!
//! Allows writing content to files on the filesystem.

use super::error::{validate_path_safety, Result, ToolError};
use super::file_read_cache::{FileFingerprint, ReadGate};
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
    /// Path to the file to write. Accepts `file_path` as an alias - the
    /// field name sent by Claude Code's and qwen-code's write tools.
    #[serde(alias = "file_path")]
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
        "Write content to a file on the filesystem. Creates the file if it doesn't exist. If it \
         does exist, this overwrites it, and you must have read it with read_file at least once \
         in this session first."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to write (absolute or relative to working directory). Alias: file_path."
                },
                "file_path": {
                    "type": "string",
                    "description": "Alias of 'path'."
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
        vec![
            ToolCapability::WriteFiles,
            ToolCapability::SystemModification,
        ]
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
        // Check if in read-only mode (Plan mode)
        if context.read_only_mode {
            return Ok(ToolResult::error(
                "Write operations are not allowed in Plan mode. \
                 Please approve the plan and switch to execution mode (Ctrl+A) to write files."
                    .to_string(),
            ));
        }

        let input: WriteInput = serde_json::from_value(input)?;

        // Enforce project boundary (T056)
        if let Err(reason) =
            crate::llm::tools::sandbox::check_path(&input.path, &context.working_directory)
        {
            return Ok(ToolResult::error(reason));
        }

        // Resolve path relative to working directory
        let path = if PathBuf::from(&input.path).is_absolute() {
            PathBuf::from(&input.path)
        } else {
            context.working_directory.join(&input.path)
        };

        // Create parent directories if requested (before path validation)
        if input.create_dirs {
            if let Some(parent) = path.parent() {
                // Validate parent path is within working directory
                let canonical_wd = context.working_directory.canonicalize().map_err(|e| {
                    ToolError::Internal(format!("Failed to canonicalize working directory: {}", e))
                })?;

                // If parent exists, check it's within bounds
                if parent.exists() {
                    let canonical_parent = parent.canonicalize().map_err(|e| {
                        ToolError::InvalidInput(format!("Failed to resolve parent path: {}", e))
                    })?;

                    if !canonical_parent.starts_with(&canonical_wd) {
                        return Ok(ToolResult::error(format!(
                            "Access denied: Path '{}' is outside the working directory",
                            input.path
                        )));
                    }
                }

                fs::create_dir_all(parent).await.map_err(ToolError::Io)?;
            }
        }

        // Validate path is safe and within working directory (prevents path traversal)
        let path = match validate_path_safety(&input.path, &context.working_directory) {
            Ok(p) => p,
            Err(ToolError::PermissionDenied(msg)) => {
                return Ok(ToolResult::error(format!("Access denied: {}", msg)));
            }
            Err(ToolError::InvalidInput(msg))
                if msg.contains("Parent directory does not exist") =>
            {
                // For write operations, we want to give a helpful error about create_dirs
                if let Some(parent) = path.parent() {
                    return Ok(ToolResult::error(format!(
                        "Parent directory does not exist: {}. Use create_dirs: true to create it.",
                        parent.display()
                    )));
                }
                return Ok(ToolResult::error(msg));
            }
            Err(ToolError::InvalidInput(msg)) => {
                return Ok(ToolResult::error(format!("Invalid path: {}", msg)));
            }
            Err(e) => return Err(e),
        };

        // Check if parent directory exists (safety check after validation)
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Ok(ToolResult::error(format!(
                    "Parent directory does not exist: {}. Use create_dirs: true to create it.",
                    parent.display()
                )));
            }
        }

        // Prior-read enforcement (matches Claude Code's/qwen-code's write
        // tools) only applies to overwriting an EXISTING file - there is
        // nothing to have read before creating a brand-new one.
        if let Ok(metadata_before) = fs::metadata(&path).await {
            match context
                .file_read_cache
                .check(&path, FileFingerprint::of(&metadata_before))
            {
                ReadGate::NeverRead => {
                    return Ok(ToolResult::error(format!(
                        "'{}' already exists. You must read it with read_file before \
                         overwriting it.",
                        path.display()
                    )));
                }
                ReadGate::Stale => {
                    return Ok(ToolResult::error(format!(
                        "'{}' has changed on disk since it was last read. Re-read it with \
                         read_file before overwriting.",
                        path.display()
                    )));
                }
                ReadGate::Ok => {}
            }
        }

        // Write the file
        fs::write(&path, &input.content)
            .await
            .map_err(ToolError::Io)?;

        // Seed the cache with the post-write fingerprint - for a new file
        // this is the read cache's first record; for an overwrite it
        // clears the way for a follow-up edit without an intervening
        // re-read (the model authored these bytes).
        let metadata_after = fs::metadata(&path).await.map_err(ToolError::Io)?;
        context
            .file_read_cache
            .record(&path, FileFingerprint::of(&metadata_after));

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

    /// Regression: Claude Code's and qwen-code's write tools send
    /// `file_path`, not `path`. A model trained on either must still be
    /// able to call this tool.
    #[tokio::test]
    async fn test_write_file_accepts_file_path_alias() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let tool = WriteTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "file_path": "test.txt",
            "content": "Hello via file_path"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success, "{:?}", result.error);

        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "Hello via file_path");
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
        tokio::fs::write(&file_path, "Initial content")
            .await
            .unwrap();

        let tool = WriteTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_working_directory(temp_dir.path().to_path_buf());

        // Overwriting an existing file requires having read it first.
        let metadata = fs::metadata(&file_path).await.unwrap();
        context
            .file_read_cache
            .record(&file_path, FileFingerprint::of(&metadata));

        let input = serde_json::json!({
            "path": "test.txt",
            "content": "New content"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success, "{:?}", result.error);

        // Verify file was overwritten
        let contents = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(contents, "New content");
    }

    /// Regression: matches Claude Code's/qwen-code's write tools - a model
    /// must not blindly overwrite a file it never read. Creating a brand
    /// new file (the common case, and every other test in this module) is
    /// unaffected - there's nothing to have read.
    #[tokio::test]
    async fn test_overwrite_rejects_a_file_never_read_this_session() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "Initial content")
            .await
            .unwrap();

        let tool = WriteTool;
        let context = ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({ "path": "test.txt", "content": "New content" });
        let result = tool.execute(input, &context).await.unwrap();

        assert!(!result.success);
        assert!(result.error.unwrap().contains("must read"));
        // Must not have touched the file.
        assert_eq!(
            tokio::fs::read_to_string(&file_path).await.unwrap(),
            "Initial content"
        );
    }

    /// A file that changed on disk after it was read must not be silently
    /// clobbered with stale assumptions.
    #[tokio::test]
    async fn test_overwrite_rejects_a_file_changed_since_it_was_read() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        tokio::fs::write(&file_path, "Initial content").await.unwrap();

        let context = ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf());
        let metadata = fs::metadata(&file_path).await.unwrap();
        context
            .file_read_cache
            .record(&file_path, FileFingerprint::of(&metadata));

        // Changes on disk after the recorded read.
        tokio::fs::write(&file_path, "Initial content, extended")
            .await
            .unwrap();

        let tool = WriteTool;
        let input = serde_json::json!({ "path": "test.txt", "content": "New content" });
        let result = tool.execute(input, &context).await.unwrap();

        assert!(!result.success);
        assert!(result.error.unwrap().contains("changed on disk"));
        assert_eq!(
            tokio::fs::read_to_string(&file_path).await.unwrap(),
            "Initial content, extended"
        );
    }

    /// Writing a brand-new file needs no prior read - the model is
    /// authoring the content, not reading existing bytes.
    #[tokio::test]
    async fn test_creating_a_new_file_needs_no_prior_read() {
        let temp_dir = TempDir::new().unwrap();
        let tool = WriteTool;
        let context = ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({ "path": "brand_new.txt", "content": "hello" });
        let result = tool.execute(input, &context).await.unwrap();

        assert!(result.success, "{:?}", result.error);
    }

    /// A write's own post-write record clears the gate for a follow-up
    /// edit_file call in the same session, without an intervening re-read.
    #[tokio::test]
    async fn test_write_then_overwrite_does_not_require_a_re_read() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let tool = WriteTool;
        let context = ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf());

        let first = tool
            .execute(
                serde_json::json!({ "path": "test.txt", "content": "first" }),
                &context,
            )
            .await
            .unwrap();
        assert!(first.success, "{:?}", first.error);

        // No re-read in between.
        let second = tool
            .execute(
                serde_json::json!({ "path": "test.txt", "content": "second" }),
                &context,
            )
            .await
            .unwrap();
        assert!(second.success, "{:?}", second.error);
        assert_eq!(
            tokio::fs::read_to_string(&file_path).await.unwrap(),
            "second"
        );
    }
}
