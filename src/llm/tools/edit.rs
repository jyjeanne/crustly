//! Edit File Tool
//!
//! Intelligently modify portions of files (find/replace, line-based edits).

use super::error::{validate_file_path, Result, ToolError};
use super::file_read_cache::{FileFingerprint, ReadGate};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;

/// Edit file tool
pub struct EditTool;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "operation")]
enum EditOperation {
    /// Replace old_text with new_text. Accepts `old_string`/`new_string` as
    /// aliases - the field names sent by Claude Code's and qwen-code's edit
    /// tools - so a model trained on either can drive this operation
    /// without knowing Crustly's names for the same fields.
    #[serde(rename = "replace")]
    Replace {
        #[serde(alias = "old_string")]
        old_text: String,
        #[serde(alias = "new_string")]
        new_text: String,
        /// Replace every occurrence of `old_text`. When false (default),
        /// `old_text` must match exactly once in the file - matching
        /// Claude Code's and qwen-code's edit tool semantics, which
        /// require enough surrounding context to make the match unique
        /// rather than silently rewriting every occurrence.
        #[serde(default)]
        replace_all: bool,
    },

    /// Replace text at specific line range
    #[serde(rename = "replace_lines")]
    ReplaceLines {
        start_line: usize,
        end_line: usize,
        new_text: String,
    },

    /// Insert text at specific line
    #[serde(rename = "insert_line")]
    InsertLine { line: usize, text: String },

    /// Delete lines
    #[serde(rename = "delete_lines")]
    DeleteLines { start_line: usize, end_line: usize },

    /// Regex replace
    #[serde(rename = "regex_replace")]
    RegexReplace {
        pattern: String,
        replacement: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
struct EditInput {
    /// Path to the file to edit. Accepts `file_path` as an alias - the
    /// field name sent by Claude Code's and qwen-code's edit tools.
    #[serde(alias = "file_path")]
    path: String,

    /// Edit operation to perform
    #[serde(flatten)]
    operation: EditOperation,

    /// Create backup before editing
    #[serde(default = "default_true")]
    create_backup: bool,
}

fn default_true() -> bool {
    true
}

/// Normalize model-provided input into the shape `EditInput` expects.
///
/// Every reference coding agent's edit tool (Claude Code, qwen-code) sends a
/// flat `{file_path, old_string, new_string, replace_all?}` payload with no
/// operation discriminator. `EditInput`'s internally-tagged `operation` enum
/// requires that discriminator to be present in the JSON to deserialize at
/// all, so without this step a model trained on either agent would fail to
/// call this tool. When `operation` is absent but `old_string`/`old_text` is
/// present, inject `"operation": "replace"` so the common case works
/// without the caller needing to know about Crustly's multi-operation
/// format.
fn normalize_input(mut input: Value) -> Value {
    if let Some(obj) = input.as_object_mut() {
        if !obj.contains_key("operation")
            && (obj.contains_key("old_string") || obj.contains_key("old_text"))
        {
            obj.insert(
                "operation".to_string(),
                Value::String("replace".to_string()),
            );
        }
    }
    input
}

#[async_trait]
impl Tool for EditTool {
    fn name(&self) -> &str {
        "edit_file"
    }

    fn description(&self) -> &str {
        "Edit a file intelligently using various operations: replace text, replace lines, insert lines, delete lines, or regex replace. \
         For a simple text replacement, `file_path`/`old_string`/`new_string` (with optional `replace_all`) may be sent with no \
         `operation` field - it defaults to 'replace'. `old_string` must match exactly once in the file unless `replace_all` is true. \
         You must read the file with read_file at least once in this session before editing it."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file to edit (alias: file_path)"
                },
                "file_path": {
                    "type": "string",
                    "description": "Alias of 'path'."
                },
                "operation": {
                    "type": "string",
                    "description": "Type of edit operation. Optional: defaults to 'replace' when old_string/old_text is present.",
                    "enum": ["replace", "replace_lines", "insert_line", "delete_lines", "regex_replace"]
                },
                "old_text": {
                    "type": "string",
                    "description": "Text to find and replace (for 'replace' operation). Alias: old_string."
                },
                "new_text": {
                    "type": "string",
                    "description": "Replacement text (for 'replace' and 'replace_lines' operations). Alias: new_string."
                },
                "old_string": {
                    "type": "string",
                    "description": "Alias of 'old_text', for the 'replace' operation."
                },
                "new_string": {
                    "type": "string",
                    "description": "Alias of 'new_text', for the 'replace' operation."
                },
                "replace_all": {
                    "type": "boolean",
                    "description": "For the 'replace' operation: replace every occurrence of old_text/old_string. Defaults to false, which requires old_text/old_string to match exactly once."
                },
                "start_line": {
                    "type": "integer",
                    "description": "Starting line number (0-indexed, for line operations)",
                    "minimum": 0
                },
                "end_line": {
                    "type": "integer",
                    "description": "Ending line number (0-indexed, inclusive, for line operations)",
                    "minimum": 0
                },
                "line": {
                    "type": "integer",
                    "description": "Line number to insert at (0-indexed, for 'insert_line')",
                    "minimum": 0
                },
                "text": {
                    "type": "string",
                    "description": "Text to insert (for 'insert_line')"
                },
                "pattern": {
                    "type": "string",
                    "description": "Regex pattern to match (for 'regex_replace')"
                },
                "replacement": {
                    "type": "string",
                    "description": "Replacement text (for 'regex_replace')"
                },
                "create_backup": {
                    "type": "boolean",
                    "description": "Create backup file before editing (default: true)",
                    "default": true
                }
            },
            "required": ["path"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![
            ToolCapability::ReadFiles,
            ToolCapability::WriteFiles,
            ToolCapability::SystemModification,
        ]
    }

    fn requires_approval(&self) -> bool {
        true // Editing files requires approval
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let _: EditInput = serde_json::from_value(normalize_input(input.clone()))
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        // Check if in read-only mode (Plan mode)
        if context.read_only_mode {
            return Ok(ToolResult::error(
                "Edit operations are not allowed in Plan mode. \
                 Please approve the plan and switch to execution mode (Ctrl+A) to edit files."
                    .to_string(),
            ));
        }

        let input: EditInput = serde_json::from_value(normalize_input(input))?;

        // Enforce project boundary (T056)
        if let Err(reason) =
            crate::llm::tools::sandbox::check_path(&input.path, &context.working_directory)
        {
            return Ok(ToolResult::error(reason));
        }

        // Validate path: safety check, existence, and file type
        let path = match validate_file_path(&input.path, &context.working_directory) {
            Ok(p) => p,
            Err(msg) => return Ok(ToolResult::error(msg)),
        };

        // Prior-read enforcement (matches Claude Code's/qwen-code's edit
        // tools): reject blindly editing a file this session never read,
        // or one that changed on disk since it was last read.
        let metadata_before = fs::metadata(&path).await.map_err(ToolError::Io)?;
        match context
            .file_read_cache
            .check(&path, FileFingerprint::of(&metadata_before))
        {
            ReadGate::NeverRead => {
                return Ok(ToolResult::error(format!(
                    "You must read '{}' with read_file before editing it.",
                    path.display()
                )));
            }
            ReadGate::Stale => {
                return Ok(ToolResult::error(format!(
                    "'{}' has changed on disk since it was last read. Re-read it with \
                     read_file before editing.",
                    path.display()
                )));
            }
            ReadGate::Ok => {}
        }

        // Read file content
        let content = fs::read_to_string(&path).await.map_err(ToolError::Io)?;

        // Create backup if requested
        if input.create_backup {
            let backup_path = path.with_extension(format!(
                "{}.backup",
                path.extension().and_then(|s| s.to_str()).unwrap_or("txt")
            ));
            fs::write(&backup_path, &content)
                .await
                .map_err(ToolError::Io)?;
        }

        // Perform edit operation
        let new_content = match input.operation {
            EditOperation::Replace {
                old_text,
                new_text,
                replace_all,
            } => {
                let occurrences = content.matches(&old_text).count();
                if occurrences == 0 {
                    return Ok(ToolResult::error(format!(
                        "Text not found in file: '{}'",
                        old_text
                    )));
                }
                if occurrences > 1 && !replace_all {
                    return Ok(ToolResult::error(format!(
                        "Text '{}' appears {} times in the file - the match must be unique. \
                         Include more surrounding context to narrow it to one occurrence, or \
                         set replace_all to true to replace every occurrence.",
                        old_text, occurrences
                    )));
                }
                content.replace(&old_text, &new_text)
            }

            EditOperation::ReplaceLines {
                start_line,
                end_line,
                new_text,
            } => {
                let lines: Vec<&str> = content.lines().collect();
                if start_line >= lines.len() || end_line >= lines.len() {
                    return Ok(ToolResult::error(format!(
                        "Line range {}-{} out of bounds (file has {} lines)",
                        start_line,
                        end_line,
                        lines.len()
                    )));
                }
                if start_line > end_line {
                    return Ok(ToolResult::error(
                        "start_line must be <= end_line".to_string(),
                    ));
                }

                let mut new_lines = Vec::new();
                new_lines.extend_from_slice(&lines[..start_line]);
                new_lines.push(&new_text);
                if end_line + 1 < lines.len() {
                    new_lines.extend_from_slice(&lines[end_line + 1..]);
                }
                new_lines.join("\n")
            }

            EditOperation::InsertLine { line, text } => {
                let lines: Vec<&str> = content.lines().collect();
                if line > lines.len() {
                    return Ok(ToolResult::error(format!(
                        "Line {} out of bounds (file has {} lines)",
                        line,
                        lines.len()
                    )));
                }

                let mut new_lines = Vec::new();
                new_lines.extend_from_slice(&lines[..line]);
                new_lines.push(&text);
                new_lines.extend_from_slice(&lines[line..]);
                new_lines.join("\n")
            }

            EditOperation::DeleteLines {
                start_line,
                end_line,
            } => {
                let lines: Vec<&str> = content.lines().collect();
                if start_line >= lines.len() || end_line >= lines.len() {
                    return Ok(ToolResult::error(format!(
                        "Line range {}-{} out of bounds (file has {} lines)",
                        start_line,
                        end_line,
                        lines.len()
                    )));
                }
                if start_line > end_line {
                    return Ok(ToolResult::error(
                        "start_line must be <= end_line".to_string(),
                    ));
                }

                let mut new_lines = Vec::new();
                new_lines.extend_from_slice(&lines[..start_line]);
                if end_line + 1 < lines.len() {
                    new_lines.extend_from_slice(&lines[end_line + 1..]);
                }
                new_lines.join("\n")
            }

            EditOperation::RegexReplace {
                pattern,
                replacement,
            } => {
                let regex = regex::Regex::new(&pattern)
                    .map_err(|e| ToolError::InvalidInput(format!("Invalid regex: {}", e)))?;

                if !regex.is_match(&content) {
                    return Ok(ToolResult::error(format!(
                        "Pattern not found in file: '{}'",
                        pattern
                    )));
                }

                regex
                    .replace_all(&content, replacement.as_str())
                    .to_string()
            }
        };

        // Write modified content
        fs::write(&path, &new_content)
            .await
            .map_err(ToolError::Io)?;

        // Seed the cache with the post-edit fingerprint so a follow-up edit
        // in the same session doesn't need an intervening re-read.
        let metadata_after = fs::metadata(&path).await.map_err(ToolError::Io)?;
        context
            .file_read_cache
            .record(&path, FileFingerprint::of(&metadata_after));

        let lines_before = content.lines().count();
        let lines_after = new_content.lines().count();

        Ok(ToolResult::success(format!(
            "Successfully edited {}. Lines: {} → {}",
            path.display(),
            lines_before,
            lines_after
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    fn context(temp_dir: &TempDir) -> ToolExecutionContext {
        ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf())
    }

    /// A context whose file-read cache already has `relative_path` recorded
    /// against its current on-disk fingerprint, simulating "the model read
    /// this file earlier in the session." Most of edit_file's own tests
    /// exist to exercise the edit logic itself, not prior-read enforcement,
    /// so they use this rather than tripping over the gate incidentally.
    async fn seeded_context(temp_dir: &TempDir, relative_path: &str) -> ToolExecutionContext {
        let ctx = context(temp_dir);
        let full_path = temp_dir.path().join(relative_path);
        let metadata = fs::metadata(&full_path).await.unwrap();
        ctx.file_read_cache
            .record(&full_path, FileFingerprint::of(&metadata));
        ctx
    }

    #[tokio::test]
    async fn test_replace_with_explicit_operation_still_works() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "hello world").await.unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "path": "test.txt",
            "operation": "replace",
            "old_text": "world",
            "new_text": "there",
            "create_backup": false
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "test.txt").await)
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert_eq!(fs::read_to_string(&file_path).await.unwrap(), "hello there");
    }

    /// Regression: Claude Code's and qwen-code's edit tools send
    /// `file_path`/`old_string`/`new_string` with no `operation` field at
    /// all. Before the alias/normalization fix, this payload would fail to
    /// deserialize entirely (missing tag) - a model trained on either agent
    /// could not call this tool.
    #[tokio::test]
    async fn test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "hello world").await.unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "file_path": "test.txt",
            "old_string": "world",
            "new_string": "there"
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "test.txt").await)
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert_eq!(fs::read_to_string(&file_path).await.unwrap(), "hello there");
    }

    /// Regression: a non-unique `old_string` with `replace_all` unset (or
    /// explicitly false) must be rejected rather than silently rewriting
    /// every occurrence - the old default behavior could corrupt unrelated
    /// code that happened to share the matched text.
    #[tokio::test]
    async fn test_replace_rejects_non_unique_match_by_default() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "foo\nfoo\nfoo\n").await.unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "file_path": "test.txt",
            "old_string": "foo",
            "new_string": "bar"
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "test.txt").await)
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("3 times"));
        // File must be untouched.
        assert_eq!(
            fs::read_to_string(&file_path).await.unwrap(),
            "foo\nfoo\nfoo\n"
        );
    }

    #[tokio::test]
    async fn test_replace_all_true_replaces_every_occurrence() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "foo\nfoo\nfoo\n").await.unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "file_path": "test.txt",
            "old_string": "foo",
            "new_string": "bar",
            "replace_all": true
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "test.txt").await)
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert_eq!(
            fs::read_to_string(&file_path).await.unwrap(),
            "bar\nbar\nbar\n"
        );
    }

    #[tokio::test]
    async fn test_replace_missing_text_errors() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "hello world").await.unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "file_path": "test.txt",
            "old_string": "not present",
            "new_string": "x"
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "test.txt").await)
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("not found"));
    }

    /// Non-replace operations must still require an explicit `operation`
    /// field - normalization only injects the default for the replace
    /// shape, so a line-based edit without `operation` is a genuine error.
    #[tokio::test]
    async fn test_line_operation_without_operation_field_is_rejected() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "one\ntwo\nthree\n").await.unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "file_path": "test.txt",
            "start_line": 0,
            "end_line": 0,
            "new_text": "ONE"
        });

        assert!(tool.validate_input(&input).is_err());
    }

    #[tokio::test]
    async fn test_replace_lines_still_works() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "one\ntwo\nthree\n").await.unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "path": "test.txt",
            "operation": "replace_lines",
            "start_line": 1,
            "end_line": 1,
            "new_text": "TWO",
            "create_backup": false
        });

        let result = tool
            .execute(input, &seeded_context(&temp_dir, "test.txt").await)
            .await
            .unwrap();
        assert!(result.success, "{:?}", result.error);
        assert_eq!(
            fs::read_to_string(&file_path).await.unwrap(),
            "one\nTWO\nthree"
        );
    }

    #[test]
    fn test_validate_input_accepts_file_path_alias() {
        let tool = EditTool;
        let input = serde_json::json!({
            "file_path": "test.txt",
            "old_string": "a",
            "new_string": "b"
        });
        assert!(tool.validate_input(&input).is_ok());
    }

    /// Regression: matches Claude Code's/qwen-code's edit tools - a file
    /// this session has never read must not be blindly editable.
    #[tokio::test]
    async fn test_edit_rejects_a_file_never_read_this_session() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("test.txt"), "hello world")
            .await
            .unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "file_path": "test.txt",
            "old_string": "world",
            "new_string": "there"
        });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("must read"));
    }

    /// Regression: a file read earlier, then changed on disk by something
    /// else (another process, a human, a build step) before the edit
    /// arrives, must not be silently overwritten with stale assumptions.
    #[tokio::test]
    async fn test_edit_rejects_a_file_changed_since_it_was_read() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "hello world").await.unwrap();

        let ctx = seeded_context(&temp_dir, "test.txt").await;

        // The file changes on disk after the recorded read.
        fs::write(&file_path, "hello world, extended")
            .await
            .unwrap();

        let tool = EditTool;
        let input = serde_json::json!({
            "file_path": "test.txt",
            "old_string": "world",
            "new_string": "there"
        });

        let result = tool.execute(input, &ctx).await.unwrap();
        assert!(!result.success);
        assert!(result.error.unwrap().contains("changed on disk"));
        // Must not have touched the file.
        assert_eq!(
            fs::read_to_string(&file_path).await.unwrap(),
            "hello world, extended"
        );
    }

    /// End-to-end: an actual read_file call (not a hand-seeded cache)
    /// clears enforcement for a subsequent edit_file call sharing the same
    /// context - proving the two tools' cache usage actually interoperate.
    #[tokio::test]
    async fn test_read_file_then_edit_file_succeeds() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "hello world").await.unwrap();
        let ctx = context(&temp_dir);

        let read_result = crate::llm::tools::read::ReadTool
            .execute(serde_json::json!({ "path": "test.txt" }), &ctx)
            .await
            .unwrap();
        assert!(read_result.success);

        let edit_result = EditTool
            .execute(
                serde_json::json!({
                    "file_path": "test.txt",
                    "old_string": "world",
                    "new_string": "there"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(edit_result.success, "{:?}", edit_result.error);
        assert_eq!(fs::read_to_string(&file_path).await.unwrap(), "hello there");
    }

    /// A second edit in the same session, on the same file, must not need
    /// an intervening re-read - the first edit's own post-write record
    /// clears the gate for the next one (matches qwen-code's recordWrite
    /// note: the model authored those bytes, so it has "seen" them).
    #[tokio::test]
    async fn test_consecutive_edits_do_not_require_a_re_read_between_them() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "one two three").await.unwrap();
        let ctx = seeded_context(&temp_dir, "test.txt").await;

        let first = EditTool
            .execute(
                serde_json::json!({
                    "file_path": "test.txt",
                    "old_string": "one",
                    "new_string": "ONE"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(first.success, "{:?}", first.error);

        // No re-read in between.
        let second = EditTool
            .execute(
                serde_json::json!({
                    "file_path": "test.txt",
                    "old_string": "two",
                    "new_string": "TWO"
                }),
                &ctx,
            )
            .await
            .unwrap();
        assert!(second.success, "{:?}", second.error);
        assert_eq!(
            fs::read_to_string(&file_path).await.unwrap(),
            "ONE TWO three"
        );
    }
}
