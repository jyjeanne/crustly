//! Bash/Shell Command Execution Tool
//!
//! Allows executing shell commands in the system.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

/// Bash execution tool
pub struct BashTool;

#[derive(Debug, Deserialize, Serialize)]
struct BashInput {
    /// Command to execute
    command: String,

    /// Optional working directory (overrides context)
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<String>,
}

#[async_trait]
impl Tool for BashTool {
    fn name(&self) -> &str {
        "bash"
    }

    fn description(&self) -> &str {
        "Execute a shell command. Returns stdout, stderr, and exit code. Use carefully as this can modify system state."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "Shell command to execute"
                },
                "working_dir": {
                    "type": "string",
                    "description": "Optional: Working directory for command execution"
                }
            },
            "required": ["command"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![
            ToolCapability::ExecuteShell,
            ToolCapability::SystemModification,
            ToolCapability::Network,
        ]
    }

    fn requires_approval(&self) -> bool {
        true // Shell execution always requires approval
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: BashInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        if input.command.trim().is_empty() {
            return Err(ToolError::InvalidInput(
                "Command cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: BashInput = serde_json::from_value(input)?;

        // Determine working directory
        let working_dir = if let Some(ref dir) = input.working_dir {
            std::path::PathBuf::from(dir)
        } else {
            context.working_directory.clone()
        };

        // Verify working directory exists
        if !working_dir.exists() {
            return Ok(ToolResult::error(format!(
                "Working directory does not exist: {}",
                working_dir.display()
            )));
        }

        // Prepare command for the current platform
        let (shell, shell_arg) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        // Execute command with timeout
        let command_future = Command::new(shell)
            .arg(shell_arg)
            .arg(&input.command)
            .current_dir(&working_dir)
            .output();

        let output = match timeout(Duration::from_secs(context.timeout_secs), command_future).await
        {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                return Ok(ToolResult::error(format!("Command execution failed: {}", e)));
            }
            Err(_) => {
                return Err(ToolError::Timeout(context.timeout_secs));
            }
        };

        // Convert output to strings
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        // Build output message
        let mut result_text = String::new();

        if !stdout.is_empty() {
            result_text.push_str("STDOUT:\n");
            result_text.push_str(&stdout);
        }

        if !stderr.is_empty() {
            if !result_text.is_empty() {
                result_text.push_str("\n\n");
            }
            result_text.push_str("STDERR:\n");
            result_text.push_str(&stderr);
        }

        if result_text.is_empty() {
            result_text = "(no output)".to_string();
        }

        let success = output.status.success();

        let result = if success {
            ToolResult::success(result_text)
        } else {
            ToolResult {
                success: false,
                output: result_text,
                error: Some(format!("Command exited with code {}", exit_code)),
                metadata: std::collections::HashMap::new(),
            }
        };

        Ok(result
            .with_metadata("exit_code".to_string(), exit_code.to_string())
            .with_metadata("working_dir".to_string(), working_dir.display().to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_bash_simple_command() {
        let tool = BashTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id).with_auto_approve(true);

        let command = if cfg!(target_os = "windows") {
            "echo Hello"
        } else {
            "echo 'Hello'"
        };

        let input = serde_json::json!({
            "command": command
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(result.success);
        assert!(result.output.contains("Hello"));
    }

    #[tokio::test]
    async fn test_bash_with_exit_code() {
        let tool = BashTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id).with_auto_approve(true);

        let command = if cfg!(target_os = "windows") {
            "exit 1"
        } else {
            "exit 1"
        };

        let input = serde_json::json!({
            "command": command
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(!result.success);
        assert_eq!(result.metadata.get("exit_code"), Some(&"1".to_string()));
    }

    #[tokio::test]
    async fn test_bash_invalid_command() {
        let tool = BashTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id).with_auto_approve(true);

        let input = serde_json::json!({
            "command": "nonexistent_command_12345"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(!result.success);
    }

    #[tokio::test]
    #[cfg(not(target_os = "windows"))] // Skip on Windows due to cmd.exe limitations
    async fn test_bash_timeout() {
        let tool = BashTool;
        let session_id = Uuid::new_v4();
        let context = ToolExecutionContext::new(session_id)
            .with_auto_approve(true)
            .with_timeout(1); // 1 second timeout

        let input = serde_json::json!({
            "command": "sleep 5"
        });

        let result = tool.execute(input, &context).await;
        assert!(result.is_err(), "Expected timeout error, got: {:?}", result);
        assert!(matches!(result.unwrap_err(), ToolError::Timeout(_)));
    }

    #[test]
    fn test_bash_tool_schema() {
        let tool = BashTool;
        assert_eq!(tool.name(), "bash");
        assert!(tool.requires_approval());

        let capabilities = tool.capabilities();
        assert!(capabilities.contains(&ToolCapability::ExecuteShell));
        assert!(capabilities.contains(&ToolCapability::SystemModification));
    }

    #[test]
    fn test_validate_empty_command() {
        let tool = BashTool;
        let input = serde_json::json!({
            "command": ""
        });

        let result = tool.validate_input(&input);
        assert!(result.is_err());
    }
}
