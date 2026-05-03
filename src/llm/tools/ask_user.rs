//! Ask User Tool
//!
//! Allows the agent to pause and ask the user a clarifying question before continuing.
//! In non-interactive (run) mode, reads the response from stdin.
//! In TUI mode, the tool emits the question via tracing and returns a placeholder;
//! full TUI integration requires extending ToolExecutionContext with an input channel.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Ask the user a clarifying question during agent execution
pub struct AskUserTool;

#[derive(Debug, Deserialize, Serialize)]
struct AskUserInput {
    /// The question to ask the user
    question: String,

    /// Optional: explain why the answer is needed
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
}

#[async_trait]
impl Tool for AskUserTool {
    fn name(&self) -> &str {
        "ask_user"
    }

    fn description(&self) -> &str {
        "Ask the user a clarifying question and wait for their response. \
         Use this when you need additional information to complete the task correctly. \
         Prefer asking one clear, specific question rather than several at once."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "question": {
                    "type": "string",
                    "description": "The question to ask the user"
                },
                "context": {
                    "type": "string",
                    "description": "Optional: explain why you need this information"
                }
            },
            "required": ["question"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![]
    }

    fn requires_approval(&self) -> bool {
        false
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: AskUserInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        if input.question.trim().is_empty() {
            return Err(ToolError::InvalidInput(
                "question cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: AskUserInput = serde_json::from_value(input)?;

        // Log the question so it appears in debug traces regardless of mode
        tracing::info!(
            session_id = %context.session_id,
            question = %input.question,
            "Agent asking user a question"
        );

        // In auto_approve mode (e.g. CI / scripted runs) return a signal the caller can detect
        if context.auto_approve {
            let msg = format!(
                "[ask_user] Question: {}{}\n(auto_approve is enabled — no interactive response available)",
                input.question,
                input
                    .context
                    .as_deref()
                    .map(|c| format!("\nContext: {}", c))
                    .unwrap_or_default()
            );
            return Ok(ToolResult::success(msg));
        }

        // Non-interactive run mode: write to stderr, read from stdin
        let prompt = match &input.context {
            Some(ctx) => format!(
                "\n[Crustly] Context: {}\n[Crustly] Question: {}\n> ",
                ctx, input.question
            ),
            None => format!("\n[Crustly] Question: {}\n> ", input.question),
        };

        // Use a blocking task to avoid blocking the async executor on stdin
        let answer = tokio::task::spawn_blocking(move || {
            use std::io::Write;
            eprint!("{}", prompt);
            std::io::stderr().flush().ok();

            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            line.trim().to_string()
        })
        .await
        .map_err(|e| ToolError::Execution(format!("Failed to read user input: {}", e)))?;

        if answer.is_empty() {
            return Ok(ToolResult::error(
                "No response provided (empty input)".to_string(),
            ));
        }

        Ok(ToolResult::success(answer.clone()).with_metadata("answer".to_string(), answer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_question() {
        let tool = AskUserTool;
        let result = tool.validate_input(&serde_json::json!({ "question": "" }));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_question() {
        let tool = AskUserTool;
        let result =
            tool.validate_input(&serde_json::json!({ "question": "Which database should I use?" }));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_with_context() {
        let tool = AskUserTool;
        let result = tool.validate_input(&serde_json::json!({
            "question": "Which database should I use?",
            "context": "I need to store user sessions"
        }));
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_auto_approve_returns_placeholder() {
        let tool = AskUserTool;
        let context = ToolExecutionContext::new(uuid::Uuid::new_v4()).with_auto_approve(true);
        let result = tool
            .execute(
                serde_json::json!({ "question": "What is your name?" }),
                &context,
            )
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.contains("[ask_user]"));
        assert!(result.output.contains("auto_approve"));
    }
}
