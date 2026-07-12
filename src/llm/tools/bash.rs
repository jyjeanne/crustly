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

/// Standard Git-for-Windows shell locations, probed when `bash` is not on PATH.
///
/// `Git\bin\bash.exe` is the supported launcher and comes first;
/// `Git\usr\bin\bash.exe` is the internal MSYS binary and is only a fallback.
#[cfg(target_os = "windows")]
const WINDOWS_BASH_FALLBACKS: &[&str] = &[
    r"C:\Program Files\Git\bin\bash.exe",
    r"C:\Program Files (x86)\Git\bin\bash.exe",
    r"C:\Program Files\Git\usr\bin\bash.exe",
];

/// Resolve the shell used to run a `bash` tool command, as `(program, arg)`.
///
/// On Unix this is `sh -c`. On Windows the model still writes POSIX (`ls -la`,
/// pipelines), so we need a real POSIX shell:
///
/// 1. `CRUSTLY_BASH`, if set - explicit override for a non-standard install.
/// 2. `bash` on PATH.
/// 3. The standard Git-for-Windows install paths. PATH alone is not enough:
///    Git ships bash but only puts it on PATH inside a Git Bash session, so a
///    Crustly launched from PowerShell or Explorer would not find it.
/// 4. Only then `cmd /C`, which cannot run POSIX and will mislead the model.
///
/// The result is computed once - this runs on every bash tool call.
fn resolve_shell() -> (String, &'static str) {
    #[cfg(target_os = "windows")]
    {
        use std::sync::OnceLock;
        static SHELL: OnceLock<(String, &'static str)> = OnceLock::new();

        SHELL
            .get_or_init(|| {
                if let Some(bash) = std::env::var_os("CRUSTLY_BASH") {
                    let path = std::path::PathBuf::from(&bash);
                    if path.is_file() {
                        return (path.to_string_lossy().into_owned(), "-c");
                    }
                    tracing::warn!("CRUSTLY_BASH is set but not a file: {:?}; ignoring", bash);
                }

                if let Ok(bash) = which::which("bash") {
                    return (bash.to_string_lossy().into_owned(), "-c");
                }

                for candidate in WINDOWS_BASH_FALLBACKS {
                    if std::path::Path::new(candidate).is_file() {
                        tracing::debug!("Using POSIX shell found off PATH: {}", candidate);
                        return ((*candidate).to_string(), "-c");
                    }
                }

                tracing::warn!(
                    "No POSIX shell found (not on PATH, not at the standard Git for \
                     Windows locations); falling back to `cmd /C`. POSIX commands such \
                     as `ls -la` will NOT work - install Git for Windows, set \
                     CRUSTLY_BASH to a bash.exe, or use the `powershell` tool instead."
                );
                ("cmd".to_string(), "/C")
            })
            .clone()
    }

    #[cfg(not(target_os = "windows"))]
    ("sh".to_string(), "-c")
}

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

/// Check if a bash command is safe for read-only mode (Plan mode)
fn is_read_only_command(command: &str) -> bool {
    let cmd_lower = command.trim().to_lowercase();

    // Check for output redirection (dangerous in read-only mode)
    if cmd_lower.contains('>') || cmd_lower.contains(">>") {
        return false;
    }

    // Check for dangerous pipe patterns (piping to tee, writing to files)
    if cmd_lower.contains("| tee") || cmd_lower.contains("|tee") {
        return false;
    }

    // Check for command substitution (can hide dangerous commands)
    if cmd_lower.contains("$(") || cmd_lower.contains("`") {
        return false;
    }

    // Check for subshell execution
    if cmd_lower.contains("bash ") || cmd_lower.contains("sh ") || cmd_lower.contains("eval ") {
        return false;
    }

    // Get the first command (before pipes or &&)
    let first_cmd = cmd_lower
        .split('|')
        .next()
        .unwrap_or(&cmd_lower)
        .split("&&")
        .next()
        .unwrap_or(&cmd_lower)
        .split(';')
        .next()
        .unwrap_or(&cmd_lower)
        .trim();

    // Get the command name (first word) - this is what we'll validate
    let cmd_name = first_cmd.split_whitespace().next().unwrap_or("");

    // List of safe read-only single commands (exact command name match)
    let safe_single_commands = [
        "ls", "cat", "head", "tail", "less", "more", "grep", "find", "tree", "file", "pwd",
        "whoami", "hostname", "date", "echo", "which", "type", "env", "printenv", "df", "du", "wc",
        "curl", "wget", "rg", "fd", "bat", "exa", "eza",
    ];

    // List of safe git subcommands (read-only)
    let safe_git_subcommands = [
        "status",
        "log",
        "diff",
        "branch",
        "show",
        "remote",
        "tag",
        "describe",
        "rev-parse",
        "config",
        "ls-files",
        "ls-tree",
        "shortlog",
        "blame",
        "reflog",
    ];

    // List of safe cargo subcommands (read-only)
    let safe_cargo_subcommands = [
        "version",
        "check",
        "clippy",
        "fmt",
        "test",
        "build",
        "doc",
        "tree",
        "metadata",
        "verify-project",
    ];

    // Check if command is in safe single commands list (exact match)
    if safe_single_commands.contains(&cmd_name) {
        return true;
    }

    // Check for git commands with safe subcommands
    if cmd_name == "git" {
        let parts: Vec<&str> = first_cmd.split_whitespace().collect();
        if parts.len() >= 2 {
            let subcommand = parts[1];
            // Check if the git subcommand is in our safe list
            return safe_git_subcommands.contains(&subcommand);
        }
        // Bare "git" command is safe (just shows help)
        return true;
    }

    // Check for cargo commands with safe subcommands
    if cmd_name == "cargo" {
        let parts: Vec<&str> = first_cmd.split_whitespace().collect();
        if parts.len() >= 2 {
            let subcommand = parts[1];
            // Check if the cargo subcommand is in our safe list
            return safe_cargo_subcommands.contains(&subcommand);
        }
        // Bare "cargo" command is safe (just shows help)
        return true;
    }

    false
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

        // Check if in read-only mode and validate command safety
        if context.read_only_mode && !is_read_only_command(&input.command) {
            return Ok(ToolResult::error(format!(
                "Command '{}' is not allowed in Plan mode (read-only). \
                 Only safe read-only commands are permitted (git status, ls, cat, grep, etc.). \
                 Please approve the plan and switch to execution mode (Ctrl+A) to run write commands.",
                input.command
            )));
        }

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

        // Pick the shell. This tool is advertised to the model as `bash`, and
        // models accordingly emit POSIX (`ls -la`, `grep -r`, pipelines). On
        // Windows those must not be handed to `cmd.exe`, which understands none
        // of them: `cmd /C ls -la` silently resolves `ls` off PATH (e.g. Git's
        // ls.exe) and runs it detached from `current_dir`, so the model gets a
        // listing of some *other* directory and reasons from garbage.
        let (shell, shell_arg) = resolve_shell();

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
                return Ok(ToolResult::error(format!(
                    "Command execution failed: {}",
                    e
                )));
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

    /// Regression: on Windows this tool ran commands through `cmd /C`, which has
    /// no `ls`/`pwd`. `cmd /C ls -la` silently resolved `ls` off PATH (Git's
    /// ls.exe) and ran it detached from `current_dir`, so the model asked for
    /// "the current folder" and was handed the user's HOME directory instead.
    /// It then reasoned from that garbage - inventing unrelated work.
    ///
    /// The tool is advertised to the model as `bash`, so it must run POSIX and
    /// must honour the working directory it was given.
    /// Regression: the first version of the Windows fix looked only at PATH.
    /// Git for Windows ships bash but only puts it on PATH inside a Git Bash
    /// session, so it resolved fine when tests were run from Git Bash and fell
    /// back to `cmd /C` for a user launching Crustly from PowerShell - the exact
    /// case the fix existed to handle.
    #[test]
    #[cfg(target_os = "windows")]
    fn windows_resolves_a_posix_shell_not_cmd() {
        let (shell, arg) = resolve_shell();
        assert_ne!(
            shell, "cmd",
            "fell back to cmd /C: no POSIX shell was found on PATH or at the \
             standard Git for Windows locations, so `ls -la` cannot work"
        );
        assert_eq!(arg, "-c");
        assert!(
            shell.to_lowercase().contains("bash"),
            "expected a bash, got {shell:?}"
        );
    }

    #[tokio::test]
    async fn bash_runs_posix_in_the_requested_working_directory() {
        let temp = tempfile::tempdir().expect("temp dir");
        let canonical = temp.path().canonicalize().expect("canonicalize");

        let mut context = ToolExecutionContext::new(Uuid::new_v4());
        context.working_directory = canonical.clone();
        context.auto_approve = true;

        let result = BashTool
            .execute(serde_json::json!({ "command": "pwd" }), &context)
            .await
            .expect("pwd runs");

        assert!(
            result.success,
            "`pwd` must run: a shell that cannot run POSIX is not `bash`. Got: {:?}",
            result.output
        );

        // Compare on the leaf: a POSIX shell on Windows reports /d/... or
        // /tmp/..., not a `D:\` path, so only the final component is portable.
        let leaf = canonical
            .file_name()
            .expect("temp dir has a name")
            .to_string_lossy()
            .into_owned();
        assert!(
            result.output.contains(&leaf),
            "`pwd` reported the wrong directory - the shell ignored current_dir. \
             expected it to contain {:?}, got {:?}",
            leaf,
            result.output
        );
    }

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

        let command = "exit 1";

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
