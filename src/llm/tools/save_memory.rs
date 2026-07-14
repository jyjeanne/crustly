//! Save Memory Tool (qwen-code/Gemini CLI compatible)
//!
//! qwen-code's `save_memory` tool (inherited from Gemini CLI, whose fork it
//! is - `memory-config.ts` re-exports `getCurrentGeminiMdFilename` and
//! friends, confirming the shared lineage) lets the model persist a fact
//! *across sessions*: `{"fact": "<self-contained statement>"}`, no other
//! fields.
//!
//! This is deliberately a real tool, not an alias to `session_context`'s
//! `add_fact` operation: that operation's store is keyed by
//! `context_{session_id}.json`, so it disappears once the session ends.
//! Pointing `save_memory` at it would make the call succeed while silently
//! breaking the one thing a model relying on "remembered" actually needs -
//! that the fact outlives this conversation. This tool instead appends to a
//! file keyed by working directory (`.crustly/memory.md`), so it persists
//! for every future session in the same project.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct SaveMemoryTool;

#[derive(Debug, Deserialize)]
struct SaveMemoryInput {
    fact: String,
}

const MEMORY_FILE: &str = "memory.md";
const MEMORY_HEADER: &str = "## Memories";

fn memory_path(working_directory: &Path) -> PathBuf {
    working_directory.join(".crustly").join(MEMORY_FILE)
}

/// Append `fact` as a new bullet under `MEMORY_HEADER`, adding the header if
/// it's missing (empty file, or a file that predates this tool). A no-op
/// (returns `existing` unchanged, with `false`) if `fact` is already present
/// verbatim, so re-remembering the same thing doesn't pile up duplicates.
fn append_fact(existing: &str, fact: &str) -> (String, bool) {
    let bullet = format!("- {fact}");

    let mut lines: Vec<String> = if existing.trim().is_empty() {
        vec![MEMORY_HEADER.to_string()]
    } else if existing.lines().next().map(str::trim) == Some(MEMORY_HEADER) {
        existing.lines().map(String::from).collect()
    } else {
        let mut v = vec![MEMORY_HEADER.to_string()];
        v.extend(existing.lines().map(String::from));
        v
    };

    if lines.iter().any(|l| l.trim() == bullet) {
        return (existing.to_string(), false);
    }

    lines.push(bullet);
    let mut result = lines.join("\n");
    result.push('\n');
    (result, true)
}

#[async_trait]
impl Tool for SaveMemoryTool {
    fn name(&self) -> &str {
        "save_memory"
    }

    fn description(&self) -> &str {
        "Save a specific, self-contained fact to long-term project memory, so it's available in \
         future sessions in this project (not just the rest of this conversation). Use for \
         durable facts worth remembering across sessions - user preferences, project \
         conventions, standing decisions. For task-scoped state that shouldn't outlive this \
         conversation, use todo_write or session_context instead."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "fact": {
                    "type": "string",
                    "description": "The specific fact to remember. Should be a clear, self-contained statement (e.g. 'The user prefers tabs over spaces', not 'use tabs')."
                }
            },
            "required": ["fact"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::WriteFiles]
    }

    fn requires_approval(&self) -> bool {
        false // Remembering a fact is low-risk, same as todo_write/session_context.
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: SaveMemoryInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;
        if input.fact.trim().is_empty() {
            return Err(ToolError::InvalidInput("fact cannot be empty".to_string()));
        }
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        // Matches todo_write's precedent: bookkeeping writes under
        // .crustly/ are still blocked in Plan mode, since Plan mode's
        // guarantee is "nothing changes until the plan is approved."
        if context.read_only_mode {
            return Err(ToolError::PermissionDenied(
                "Cannot save memory in read-only (plan) mode".to_string(),
            ));
        }

        let input: SaveMemoryInput = serde_json::from_value(input)?;
        let fact = input.fact.trim();
        if fact.is_empty() {
            return Ok(ToolResult::error("fact cannot be empty".to_string()));
        }

        let path = memory_path(&context.working_directory);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(ToolError::Io)?;
        }

        let existing = if path.exists() {
            fs::read_to_string(&path).await.map_err(ToolError::Io)?
        } else {
            String::new()
        };

        let (new_content, added) = append_fact(&existing, fact);
        if added {
            fs::write(&path, &new_content).await.map_err(ToolError::Io)?;
        }

        let message = if added {
            format!("Remembered: {fact}")
        } else {
            format!("Already remembered: {fact}")
        };

        Ok(ToolResult::success(message)
            .with_metadata("path".to_string(), path.display().to_string()))
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

    #[tokio::test]
    async fn execute_creates_memory_file_with_header_and_fact() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool;

        let result = tool
            .execute(
                serde_json::json!({ "fact": "The user prefers tabs over spaces" }),
                &context(&temp_dir),
            )
            .await
            .unwrap();

        assert!(result.success, "{:?}", result.error);
        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert_eq!(
            contents,
            "## Memories\n- The user prefers tabs over spaces\n"
        );
    }

    #[tokio::test]
    async fn execute_appends_to_existing_memory_file() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool;
        let ctx = context(&temp_dir);

        tool.execute(serde_json::json!({ "fact": "first fact" }), &ctx)
            .await
            .unwrap();
        tool.execute(serde_json::json!({ "fact": "second fact" }), &ctx)
            .await
            .unwrap();

        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert_eq!(contents, "## Memories\n- first fact\n- second fact\n");
    }

    /// Regression: re-remembering the exact same fact must not duplicate it.
    #[tokio::test]
    async fn execute_does_not_duplicate_an_identical_fact() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool;
        let ctx = context(&temp_dir);

        tool.execute(serde_json::json!({ "fact": "same fact" }), &ctx)
            .await
            .unwrap();
        let second = tool
            .execute(serde_json::json!({ "fact": "same fact" }), &ctx)
            .await
            .unwrap();

        assert!(second.success);
        assert!(second.output.contains("Already remembered"));
        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert_eq!(contents, "## Memories\n- same fact\n");
    }

    /// Regression: this is the whole point of the tool over session_context's
    /// add_fact - the memory file is keyed by working directory, not session
    /// id, so a second "session" (a fresh ToolExecutionContext, same working
    /// directory) sees facts saved by an earlier one.
    #[tokio::test]
    async fn memory_persists_across_different_sessions_in_the_same_directory() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool;

        let session_one = context(&temp_dir);
        tool.execute(
            serde_json::json!({ "fact": "remembered in session one" }),
            &session_one,
        )
        .await
        .unwrap();

        // A different session_id (a fresh conversation), same directory.
        let session_two = context(&temp_dir);
        assert_ne!(session_one.session_id, session_two.session_id);

        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert!(contents.contains("remembered in session one"));

        tool.execute(
            serde_json::json!({ "fact": "remembered in session two" }),
            &session_two,
        )
        .await
        .unwrap();

        let contents = tokio::fs::read_to_string(temp_dir.path().join(".crustly/memory.md"))
            .await
            .unwrap();
        assert!(contents.contains("remembered in session one"));
        assert!(contents.contains("remembered in session two"));
    }

    #[tokio::test]
    async fn execute_blocked_in_read_only_mode() {
        let temp_dir = TempDir::new().unwrap();
        let tool = SaveMemoryTool;
        let mut ctx = context(&temp_dir);
        ctx.read_only_mode = true;

        let result = tool
            .execute(serde_json::json!({ "fact": "x" }), &ctx)
            .await;
        assert!(matches!(result, Err(ToolError::PermissionDenied(_))));
    }

    #[test]
    fn validate_input_rejects_empty_fact() {
        let tool = SaveMemoryTool;
        assert!(tool
            .validate_input(&serde_json::json!({ "fact": "   " }))
            .is_err());
    }

    #[test]
    fn append_fact_adds_header_to_a_file_that_lacks_one() {
        let (content, added) = append_fact("- pre-existing line without a header\n", "new fact");
        assert!(added);
        assert_eq!(
            content,
            "## Memories\n- pre-existing line without a header\n- new fact\n"
        );
    }
}
