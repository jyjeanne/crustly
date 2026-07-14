//! Glob Pattern Matching Tool
//!
//! Find files matching glob patterns.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

/// Glob pattern matching tool
pub struct GlobTool;

#[derive(Debug, Deserialize, Serialize)]
struct GlobInput {
    /// Glob pattern to match
    pattern: String,

    /// Base directory for search (defaults to working directory)
    #[serde(default)]
    base_dir: Option<String>,

    /// Maximum number of results to return
    #[serde(default)]
    limit: Option<usize>,

    /// Include hidden files
    #[serde(default)]
    include_hidden: bool,
}

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &str {
        "glob"
    }

    fn description(&self) -> &str {
        "Find files matching a glob pattern. Supports wildcards: * (any chars), ** (recursive directories), ? (single char), [abc] (char class)."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "Glob pattern (e.g., '**/*.rs', 'src/**/*.test.js', '*.{md,txt}')"
                },
                "base_dir": {
                    "type": "string",
                    "description": "Base directory for search (defaults to working directory)"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of results to return",
                    "minimum": 1
                },
                "include_hidden": {
                    "type": "boolean",
                    "description": "Include hidden files (starting with .)",
                    "default": false
                }
            },
            "required": ["pattern"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ReadFiles]
    }

    fn requires_approval(&self) -> bool {
        false // Pattern matching is safe
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: GlobInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        if input.pattern.trim().is_empty() {
            return Err(ToolError::InvalidInput(
                "Pattern cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: GlobInput = serde_json::from_value(input)?;

        // Enforce project boundary on explicit base_dir (T056)
        if let Some(ref dir) = input.base_dir {
            if let Err(reason) =
                crate::llm::tools::sandbox::check_path(dir, &context.working_directory)
            {
                return Ok(ToolResult::error(reason));
            }
        }

        // Resolve base directory
        let base_dir = if let Some(ref dir) = input.base_dir {
            if PathBuf::from(dir).is_absolute() {
                PathBuf::from(dir)
            } else {
                context.working_directory.join(dir)
            }
        } else {
            context.working_directory.clone()
        };

        if !base_dir.exists() {
            return Ok(ToolResult::error(format!(
                "Base directory does not exist: {}",
                base_dir.display()
            )));
        }

        let glob_pattern = glob::Pattern::new(&input.pattern)
            .map_err(|e| ToolError::InvalidInput(format!("Invalid glob pattern: {}", e)))?;

        // Enumerate candidate files via a gitignore-aware walk (matches
        // ripgrep's - and therefore qwen-code's `glob` and Claude Code's
        // `Glob` - default behavior: `**/*.rs` should not wade into
        // target/, node_modules/, etc.), then filter by the glob pattern
        // relative to base_dir. The walk itself is blocking I/O, so it
        // runs on a blocking thread; only directory traversal moves there,
        // not the (cheap, in-memory) pattern matching below.
        let dir_for_walk = base_dir.clone();
        let include_hidden = input.include_hidden;
        let candidates: Vec<PathBuf> = tokio::task::spawn_blocking(move || {
            ignore::WalkBuilder::new(&dir_for_walk)
                .hidden(!include_hidden)
                .build()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
                .map(|entry| entry.into_path())
                .collect::<Vec<_>>()
        })
        .await
        .map_err(|e| ToolError::Internal(format!("directory walk failed: {e}")))?;

        let mut matches: Vec<PathBuf> = candidates
            .into_iter()
            .filter(|path| {
                let rel = path.strip_prefix(&base_dir).unwrap_or(path);
                glob_pattern.matches_path(rel)
            })
            .collect();

        if let Some(limit) = input.limit {
            matches.truncate(limit);
        }

        if matches.is_empty() {
            return Ok(ToolResult::success(format!(
                "No files found matching pattern: {}",
                input.pattern
            )));
        }

        // Sort matches for consistent output
        matches.sort();

        // Format output
        let mut output = format!(
            "Found {} files matching '{}':\n\n",
            matches.len(),
            input.pattern
        );

        for path in &matches {
            // Make path relative to base_dir for cleaner output
            let display_path = path
                .strip_prefix(&base_dir)
                .unwrap_or(path)
                .display()
                .to_string();
            output.push_str(&format!("  {}\n", display_path));
        }

        if let Some(limit) = input.limit {
            if matches.len() >= limit {
                output.push_str(&format!("\n(Limited to {} results)", limit));
            }
        }

        Ok(ToolResult::success(output))
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
    async fn test_glob_matches_recursive_pattern() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::create_dir_all(temp_dir.path().join("src/nested"))
            .await
            .unwrap();
        tokio::fs::write(temp_dir.path().join("src/a.rs"), "").await.unwrap();
        tokio::fs::write(temp_dir.path().join("src/nested/b.rs"), "")
            .await
            .unwrap();
        tokio::fs::write(temp_dir.path().join("readme.md"), "").await.unwrap();

        let tool = GlobTool;
        let input = serde_json::json!({ "pattern": "**/*.rs" });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(result.output.contains("a.rs"));
        assert!(result.output.contains("b.rs"));
        assert!(!result.output.contains("readme.md"));
    }

    /// Regression: an unbounded `**` glob must not wade into directories
    /// the project's `.gitignore` excludes - both qwen-code's `glob` and
    /// Claude Code's `Glob` are ripgrep-backed and respect `.gitignore` by
    /// default.
    #[tokio::test]
    async fn test_glob_respects_gitignore() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join(".gitignore"), "target/\n")
            .await
            .unwrap();
        tokio::fs::create_dir(temp_dir.path().join(".git")).await.unwrap();
        tokio::fs::create_dir_all(temp_dir.path().join("target/debug"))
            .await
            .unwrap();
        tokio::fs::write(temp_dir.path().join("target/debug/build.rs"), "")
            .await
            .unwrap();
        tokio::fs::write(temp_dir.path().join("src.rs"), "").await.unwrap();

        let tool = GlobTool;
        let input = serde_json::json!({ "pattern": "**/*.rs" });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(result.output.contains("src.rs"));
        assert!(
            !result.output.contains("build.rs"),
            "gitignored target/ must be skipped, got: {:?}",
            result.output
        );
    }

    #[tokio::test]
    async fn test_glob_no_matches() {
        let temp_dir = TempDir::new().unwrap();
        let tool = GlobTool;
        let input = serde_json::json!({ "pattern": "**/*.nonexistent" });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("No files found"));
    }

    #[tokio::test]
    async fn test_glob_respects_limit() {
        let temp_dir = TempDir::new().unwrap();
        for i in 0..5 {
            tokio::fs::write(temp_dir.path().join(format!("f{i}.txt")), "")
                .await
                .unwrap();
        }

        let tool = GlobTool;
        let input = serde_json::json!({ "pattern": "*.txt", "limit": 2 });

        let result = tool.execute(input, &context(&temp_dir)).await.unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(result.output.contains("Found 2 files"));
    }
}
