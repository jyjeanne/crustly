//! Grep Content Search Tool
//!
//! Search file contents for matching patterns.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Grep search tool
pub struct GrepTool;

#[derive(Debug, Deserialize, Serialize)]
struct GrepInput {
    /// Pattern to search for
    pattern: String,

    /// Path to search (file or directory)
    #[serde(default)]
    path: Option<String>,

    /// Whether to treat `pattern` as a regex. Defaults to true - Claude
    /// Code's and qwen-code's grep tools always treat pattern as a full
    /// regex with no literal/regex toggle at all, so a model trained on
    /// either never sends this field and expects regex semantics by
    /// default. Set to false to search for `pattern` as a literal string
    /// instead (regex metacharacters are escaped).
    #[serde(default = "default_true")]
    regex: bool,

    /// Case insensitive search
    #[serde(default)]
    case_insensitive: bool,

    /// Show line numbers
    #[serde(default = "default_true")]
    line_numbers: bool,

    /// Context lines to show before and after match
    #[serde(default)]
    context: Option<usize>,

    /// File pattern to filter (e.g., "*.rs"). Accepts `glob` as an alias -
    /// the field name sent by Claude Code's and qwen-code's grep tools.
    #[serde(alias = "glob", default)]
    file_pattern: Option<String>,

    /// Maximum number of matches to return
    #[serde(default)]
    limit: Option<usize>,
}

fn default_true() -> bool {
    true
}

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search for patterns in file contents. Supports full regex syntax (e.g. \"log.*Error\", \"function\\s+\\w+\") with context lines. Set regex: false to search for pattern as a literal string instead."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The regular expression pattern to search for in file contents"
                },
                "path": {
                    "type": "string",
                    "description": "File or directory to search (defaults to working directory)"
                },
                "regex": {
                    "type": "boolean",
                    "description": "Whether to treat pattern as a regex. Defaults to true; set to false to search for pattern as a literal string instead.",
                    "default": true
                },
                "case_insensitive": {
                    "type": "boolean",
                    "description": "Case insensitive search",
                    "default": false
                },
                "line_numbers": {
                    "type": "boolean",
                    "description": "Show line numbers in results",
                    "default": true
                },
                "context": {
                    "type": "integer",
                    "description": "Number of context lines to show before and after match",
                    "minimum": 0
                },
                "file_pattern": {
                    "type": "string",
                    "description": "File pattern to filter (e.g., '*.rs', '*.{js,ts}'). Alias: glob."
                },
                "glob": {
                    "type": "string",
                    "description": "Alias of 'file_pattern'."
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of matches to return",
                    "minimum": 1
                }
            },
            "required": ["pattern"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::ReadFiles]
    }

    fn requires_approval(&self) -> bool {
        false // Searching is safe
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: GrepInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        if input.pattern.trim().is_empty() {
            return Err(ToolError::InvalidInput(
                "Pattern cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: GrepInput = serde_json::from_value(input)?;

        // Enforce project boundary on explicit path (T056)
        if let Some(ref p) = input.path {
            if let Err(reason) =
                crate::llm::tools::sandbox::check_path(p, &context.working_directory)
            {
                return Ok(ToolResult::error(reason));
            }
        }

        // Build regex pattern
        let pattern_str = if input.regex {
            input.pattern.clone()
        } else {
            regex::escape(&input.pattern)
        };

        let regex = if input.case_insensitive {
            regex::RegexBuilder::new(&pattern_str)
                .case_insensitive(true)
                .build()
        } else {
            regex::Regex::new(&pattern_str)
        }
        .map_err(|e| ToolError::InvalidInput(format!("Invalid pattern: {}", e)))?;

        // Resolve search path
        let search_path = if let Some(ref p) = input.path {
            if PathBuf::from(p).is_absolute() {
                PathBuf::from(p)
            } else {
                context.working_directory.join(p)
            }
        } else {
            context.working_directory.clone()
        };

        if !search_path.exists() {
            return Ok(ToolResult::error(format!(
                "Path does not exist: {}",
                search_path.display()
            )));
        }

        let mut matches = Vec::new();
        let mut total_matches = 0;

        if search_path.is_file() {
            self.search_file(
                &search_path,
                &regex,
                &input,
                &mut matches,
                &mut total_matches,
            )
            .await?;
        } else {
            self.search_directory(
                &search_path,
                &regex,
                &input,
                &mut matches,
                &mut total_matches,
            )
            .await?;
        }

        if matches.is_empty() {
            return Ok(ToolResult::success(format!(
                "No matches found for pattern: '{}'",
                input.pattern
            )));
        }

        let output = matches.join("\n\n");
        let summary = if let Some(_limit) = input.limit {
            if total_matches > matches.len() {
                format!(
                    "\n\n({} matches shown, {} total)",
                    matches.len(),
                    total_matches
                )
            } else {
                format!("\n\n({} matches)", total_matches)
            }
        } else {
            format!("\n\n({} matches)", total_matches)
        };

        Ok(ToolResult::success(format!("{}{}", output, summary)))
    }
}

impl GrepTool {
    async fn search_file(
        &self,
        path: &Path,
        regex: &regex::Regex,
        input: &GrepInput,
        matches: &mut Vec<String>,
        total_matches: &mut usize,
    ) -> Result<()> {
        // Check file pattern filter
        if let Some(ref pattern) = input.file_pattern {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let glob_pattern = glob::Pattern::new(pattern)
                .map_err(|e| ToolError::InvalidInput(format!("Invalid file pattern: {}", e)))?;

            if !glob_pattern.matches(file_name) {
                return Ok(());
            }
        }

        let content = match fs::read_to_string(path).await {
            Ok(c) => c,
            Err(_) => return Ok(()), // Skip binary files or unreadable files
        };

        let lines: Vec<&str> = content.lines().collect();
        let display_path = path.display().to_string();

        for (line_num, line) in lines.iter().enumerate() {
            if regex.is_match(line) {
                *total_matches += 1;

                // Check limit
                if let Some(limit) = input.limit {
                    if matches.len() >= limit {
                        return Ok(());
                    }
                }

                let mut result = String::new();
                result.push_str(&format!("{}:", display_path));

                if input.line_numbers {
                    result.push_str(&format!("{}:", line_num + 1));
                }

                // Add context before
                if let Some(ctx) = input.context {
                    let start = line_num.saturating_sub(ctx);
                    for (i, line) in lines.iter().enumerate().skip(start).take(line_num - start) {
                        result.push_str(&format!("\n  {}: {}", i + 1, line));
                    }
                }

                // Add matching line
                result.push_str(&format!("\n> {}", line));

                // Add context after
                if let Some(ctx) = input.context {
                    let end = (line_num + ctx + 1).min(lines.len());
                    for (i, line) in lines
                        .iter()
                        .enumerate()
                        .skip(line_num + 1)
                        .take(end - line_num - 1)
                    {
                        result.push_str(&format!("\n  {}: {}", i + 1, line));
                    }
                }

                matches.push(result);
            }
        }

        Ok(())
    }

    fn search_directory<'a>(
        &'a self,
        dir: &'a PathBuf,
        regex: &'a regex::Regex,
        input: &'a GrepInput,
        matches: &'a mut Vec<String>,
        total_matches: &'a mut usize,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(dir).await.map_err(ToolError::Io)?;

            while let Some(entry) = entries.next_entry().await.map_err(ToolError::Io)? {
                let path = entry.path();

                // Check limit
                if let Some(limit) = input.limit {
                    if matches.len() >= limit {
                        return Ok(());
                    }
                }

                if path.is_file() {
                    self.search_file(&path, regex, input, matches, total_matches)
                        .await?;
                } else if path.is_dir() {
                    // Skip hidden directories
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with('.') {
                            continue;
                        }
                    }
                    self.search_directory(&path, regex, input, matches, total_matches)
                        .await?;
                }
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    /// Regression: Claude Code's and qwen-code's grep tools send `glob`,
    /// not `file_pattern`. A model trained on either must still be able to
    /// filter by file pattern.
    #[tokio::test]
    async fn test_grep_accepts_glob_alias_for_file_pattern() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("match.rs"), "needle here")
            .await
            .unwrap();
        tokio::fs::write(temp_dir.path().join("match.txt"), "needle here")
            .await
            .unwrap();

        let tool = GrepTool;
        let context = ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({
            "pattern": "needle",
            "glob": "*.rs"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(result.output.contains("match.rs"));
        assert!(!result.output.contains("match.txt"));
    }

    /// Regression: Claude Code's and qwen-code's grep tools always treat
    /// `pattern` as a regex - there's no literal/regex toggle in either -
    /// so a model trained on either never sends `regex` and expects regex
    /// semantics by default. Before this fix, an omitted `regex` field
    /// defaulted to literal-string matching, so `.` in a pattern like
    /// `fn.run` would only match a literal dot instead of "any character",
    /// silently missing matches a Claude Code/qwen-code-trained model
    /// expects to find.
    #[tokio::test]
    async fn test_pattern_is_regex_by_default() {
        let temp_dir = TempDir::new().unwrap();
        // "fn.run" as a regex matches this via `.` = "any character"; as a
        // literal string it would not (no literal dot between fn and run).
        tokio::fs::write(temp_dir.path().join("code.rs"), "fnXrun")
            .await
            .unwrap();

        let tool = GrepTool;
        let context = ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf());

        // No `regex` field at all - matches what a Claude Code/qwen-code-
        // trained model actually sends.
        let input = serde_json::json!({ "pattern": "fn.run" });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(
            result.output.contains("code.rs"),
            "expected regex '.' to match any character by default, got: {:?}",
            result.output
        );
    }

    /// `regex: false` must still work as an explicit opt-out into literal
    /// string matching (regex metacharacters escaped).
    #[tokio::test]
    async fn test_regex_false_still_searches_literally() {
        let temp_dir = TempDir::new().unwrap();
        tokio::fs::write(temp_dir.path().join("code.rs"), "fnXrun\nfn.run")
            .await
            .unwrap();

        let tool = GrepTool;
        let context = ToolExecutionContext::new(Uuid::new_v4())
            .with_working_directory(temp_dir.path().to_path_buf());

        let input = serde_json::json!({ "pattern": "fn.run", "regex": false });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success, "{:?}", result.error);
        assert!(
            result.output.contains("fn.run"),
            "literal '.' must match the literal dot"
        );
        assert!(
            !result.output.contains("fnXrun"),
            "with regex: false, '.' must not match an arbitrary character, got: {:?}",
            result.output
        );
    }
}
