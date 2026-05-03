//! PowerShell Tool
//!
//! Executes PowerShell commands on any platform where PowerShell is installed.
//! Prefers `pwsh` (PowerShell Core 6+, cross-platform) and falls back to
//! `powershell` (Windows PowerShell 5.x). Mirrors the BashTool interface and
//! adds background-execution support.
//!
//! Detection is performed once at first use (via `once_cell::sync::Lazy`) so
//! the expensive PATH probe does not repeat on every call.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

// ── Shell detection ────────────────────────────────────────────────────────────

/// Detected PowerShell executable name, resolved once at first use.
static PS_EXECUTABLE: Lazy<Option<&'static str>> = Lazy::new(|| {
    ["pwsh", "powershell"]
        .into_iter()
        .find(|&candidate| probe_executable(candidate))
});

/// Check whether `cmd` exists on PATH using a fast, side-effect-free probe.
fn probe_executable(cmd: &str) -> bool {
    if cfg!(windows) {
        // `cmd /C where <name>` is ~5 ms; much cheaper than starting PowerShell.
        std::process::Command::new("cmd")
            .args(["/C", &format!("where {cmd} >NUL 2>&1")])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    } else {
        std::process::Command::new("sh")
            .args(["-c", &format!("command -v {cmd} >/dev/null 2>&1")])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}

// ── Read-only mode ─────────────────────────────────────────────────────────────

/// Cmdlets (lowercase) considered safe for Plan mode (read-only).
const READ_ONLY_CMDLETS: &[&str] = &[
    // Filesystem reads
    "get-content",
    "get-childitem",
    "get-item",
    "get-itemproperty",
    "get-filehash",
    "select-string",
    "test-path",
    // Output / formatting
    "write-output",
    "write-host",
    "out-string",
    "format-list",
    "format-table",
    "format-wide",
    // System information
    "get-command",
    "get-help",
    "get-module",
    "get-process",
    "get-variable",
    // Pipeline helpers
    "where-object",
    "select-object",
    "sort-object",
    "group-object",
    "measure-object",
    "compare-object",
    // Data conversion (pure)
    "convertto-json",
    "convertfrom-json",
    "convertto-csv",
    "convertfrom-csv",
    // Common aliases for the above
    "dir",
    "ls",
    "cat",
    "gc",
    "gci",
    "gi",
    "echo",
    "measure",
    "where",
    "select",
    "sort",
];

/// Patterns that indicate dangerous operations regardless of cmdlet name.
const DANGEROUS_PATTERNS: &[&str] = &[
    "invoke-expression",
    "iex", // alias for Invoke-Expression (covers iex(...), iex"...", iex $var)
    " >",  // output redirection (space before > avoids false positives on ">" in string args)
    ">>",  // append redirection without spaces (e.g. cmd>>file)
    "|out-file",
    "| out-file",
    "|set-content",
    "| set-content",
    "|add-content",
    "| add-content",
    "remove-item",
    "set-item",
    "new-item",
    "rename-item",
    "copy-item",
    "move-item",
    "start-process",
    "invoke-command",
    "invoke-webrequest",
    "invoke-restmethod",
    "[system.", // direct .NET static method calls
    "[io.",
    "&(", // call operator on subexpression
    ".(", // dot-sourcing a subexpression
];

/// Return `true` if `command` is safe to run in Plan (read-only) mode.
fn is_read_only_powershell(command: &str) -> bool {
    let cmd_lower = command.trim().to_lowercase();
    if DANGEROUS_PATTERNS.iter().any(|p| cmd_lower.contains(p)) {
        return false;
    }
    READ_ONLY_CMDLETS.iter().any(|c| cmd_lower.starts_with(c))
}

// ── Tool struct & input ────────────────────────────────────────────────────────

pub struct PowerShellTool;

#[derive(Debug, Deserialize, Serialize)]
struct PowerShellInput {
    /// PowerShell command or script block to execute
    command: String,

    /// Optional working directory override (defaults to context working_directory)
    #[serde(skip_serializing_if = "Option::is_none")]
    working_dir: Option<String>,

    /// Timeout in seconds; overrides context.timeout_secs (max 600)
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_secs: Option<u64>,

    /// Fire-and-forget: spawn the process and return its PID immediately
    #[serde(default)]
    run_in_background: bool,

    /// Human-readable purpose shown in approval prompts (not sent to PowerShell)
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

// ── Tool impl ─────────────────────────────────────────────────────────────────

#[async_trait]
impl Tool for PowerShellTool {
    fn name(&self) -> &str {
        "powershell"
    }

    fn description(&self) -> &str {
        "Execute a PowerShell (pwsh / powershell.exe) command and return stdout, stderr, \
         and exit code. Supports background execution via run_in_background. \
         Requires pwsh (PowerShell Core) or powershell.exe to be on PATH."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "PowerShell command or script block to execute"
                },
                "working_dir": {
                    "type": "string",
                    "description": "Optional working directory override"
                },
                "timeout_secs": {
                    "type": "integer",
                    "description": "Timeout in seconds (default: from context, max: 600)",
                    "minimum": 1,
                    "maximum": 600
                },
                "run_in_background": {
                    "type": "boolean",
                    "description": "Spawn the command in background and return its PID immediately",
                    "default": false
                },
                "description": {
                    "type": "string",
                    "description": "Human-readable purpose shown in approval prompts"
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
        true
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: PowerShellInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        if input.command.trim().is_empty() {
            return Err(ToolError::InvalidInput(
                "command cannot be empty".to_string(),
            ));
        }
        if let Some(t) = input.timeout_secs {
            if t == 0 || t > 600 {
                return Err(ToolError::InvalidInput(
                    "timeout_secs must be between 1 and 600".to_string(),
                ));
            }
        }
        Ok(())
    }

    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: PowerShellInput = serde_json::from_value(input)?;

        // Plan mode: only allow read-only cmdlets
        if context.read_only_mode && !is_read_only_powershell(&input.command) {
            return Ok(ToolResult::error(format!(
                "PowerShell command '{}' is not allowed in Plan mode (read-only). \
                 Only safe read-only cmdlets are permitted (Get-Content, Get-ChildItem, \
                 Select-String, etc.). Approve the plan and switch to execution mode to \
                 run write commands.",
                input.command
            )));
        }

        // Resolve the PowerShell executable (cached after first use).
        // Force initialization in a blocking thread so the ~5ms PATH probe does not
        // block the Tokio worker thread on the very first call.
        if once_cell::sync::Lazy::get(&PS_EXECUTABLE).is_none() {
            tokio::task::spawn_blocking(|| {
                let _ = &*PS_EXECUTABLE;
            })
            .await
            .map_err(|e| ToolError::Execution(format!("PS detection task panicked: {e}")))?;
        }
        let shell = (*PS_EXECUTABLE).ok_or_else(|| {
            ToolError::Execution(
                "PowerShell not found. Install PowerShell Core (pwsh) or ensure \
                 powershell.exe is on PATH."
                    .to_string(),
            )
        })?;

        // Resolve working directory
        let working_dir = if let Some(ref dir) = input.working_dir {
            std::path::PathBuf::from(dir)
        } else {
            context.working_directory.clone()
        };
        if !working_dir.exists() {
            return Ok(ToolResult::error(format!(
                "Working directory does not exist: {}",
                working_dir.display()
            )));
        }

        let ps_args = ["-NoProfile", "-NonInteractive", "-Command"];

        // ── Background execution ──────────────────────────────────────────────
        if input.run_in_background {
            // Use std::process::Command so the child detaches cleanly on all platforms.
            // Dropping std::process::Child does NOT kill the process.
            let child = std::process::Command::new(shell)
                .args(ps_args)
                .arg(&input.command)
                .current_dir(&working_dir)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .map_err(|e| {
                    ToolError::Execution(format!("Failed to spawn background process: {e}"))
                })?;

            let pid = child.id();
            // Drop without waiting — the process keeps running independently.
            drop(child);

            return Ok(
                ToolResult::success(format!("Command started in background (PID: {pid})."))
                    .with_metadata("pid".to_string(), pid.to_string())
                    .with_metadata("background".to_string(), "true".to_string())
                    .with_metadata("shell".to_string(), shell.to_string()),
            );
        }

        // ── Foreground execution with timeout ─────────────────────────────────
        let timeout_secs = input.timeout_secs.unwrap_or(context.timeout_secs).min(600);

        let command_future = Command::new(shell)
            .args(ps_args)
            .arg(&input.command)
            .current_dir(&working_dir)
            .output();

        let output = match timeout(Duration::from_secs(timeout_secs), command_future).await {
            Ok(Ok(out)) => out,
            Ok(Err(e)) => return Ok(ToolResult::error(format!("Execution failed: {e}"))),
            Err(_) => return Err(ToolError::Timeout(timeout_secs)),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        let mut text = String::new();
        if !stdout.is_empty() {
            text.push_str("STDOUT:\n");
            text.push_str(&stdout);
        }
        if !stderr.is_empty() {
            if !text.is_empty() {
                text.push('\n');
            }
            text.push_str("STDERR:\n");
            text.push_str(&stderr);
        }
        if text.is_empty() {
            text = "(no output)".to_string();
        }

        let result = if output.status.success() {
            ToolResult::success(text)
        } else {
            ToolResult {
                success: false,
                output: text,
                error: Some(format!("Command exited with code {exit_code}")),
                metadata: HashMap::new(),
            }
        };

        Ok(result
            .with_metadata("exit_code".to_string(), exit_code.to_string())
            .with_metadata("shell".to_string(), shell.to_string())
            .with_metadata("working_dir".to_string(), working_dir.display().to_string()))
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn make_ctx() -> ToolExecutionContext {
        ToolExecutionContext::new(Uuid::new_v4()).with_auto_approve(true)
    }

    // ── is_read_only_powershell ────────────────────────────────────────────────

    #[test]
    fn read_only_allows_get_content() {
        assert!(is_read_only_powershell("Get-Content ./README.md"));
    }

    #[test]
    fn read_only_allows_get_childitem() {
        assert!(is_read_only_powershell("Get-ChildItem -Path . -Recurse"));
    }

    #[test]
    fn read_only_allows_select_string() {
        assert!(is_read_only_powershell(
            "Select-String -Pattern 'fn ' -Path *.rs"
        ));
    }

    #[test]
    fn read_only_blocks_remove_item() {
        assert!(!is_read_only_powershell("Remove-Item ./target -Recurse"));
    }

    #[test]
    fn read_only_blocks_invoke_expression() {
        assert!(!is_read_only_powershell("Invoke-Expression $cmd"));
    }

    #[test]
    fn read_only_blocks_pipe_to_out_file() {
        assert!(!is_read_only_powershell("Get-ChildItem |Out-File list.txt"));
    }

    #[test]
    fn read_only_blocks_net_method_call() {
        assert!(!is_read_only_powershell(
            "[System.IO.File]::WriteAllText('x', 'y')"
        ));
    }

    #[test]
    fn read_only_blocks_iex_without_space() {
        // iex($cmd) — no space between alias and argument
        assert!(!is_read_only_powershell("iex($env:PAYLOAD)"));
    }

    #[test]
    fn read_only_allows_gt_in_string_argument() {
        // ">" inside a quoted argument must not be treated as redirection
        assert!(is_read_only_powershell(
            r#"Select-String -Pattern ">" -Path *.rs"#
        ));
    }

    #[test]
    fn read_only_blocks_redirection_with_space() {
        // space before > is the canonical redirection form
        assert!(!is_read_only_powershell("Get-Content file.txt > out.txt"));
    }

    #[test]
    fn read_only_blocks_append_no_spaces() {
        // >> without surrounding spaces (e.g. cmd>>file)
        assert!(!is_read_only_powershell("Get-Content file.txt>>out.txt"));
    }

    // ── validate_input ─────────────────────────────────────────────────────────

    #[test]
    fn validate_rejects_empty_command() {
        let tool = PowerShellTool;
        assert!(tool
            .validate_input(&serde_json::json!({ "command": "  " }))
            .is_err());
    }

    #[test]
    fn validate_rejects_zero_timeout() {
        let tool = PowerShellTool;
        assert!(tool
            .validate_input(&serde_json::json!({ "command": "echo hi", "timeout_secs": 0 }))
            .is_err());
    }

    #[test]
    fn validate_rejects_timeout_over_600() {
        let tool = PowerShellTool;
        assert!(tool
            .validate_input(&serde_json::json!({ "command": "echo hi", "timeout_secs": 601 }))
            .is_err());
    }

    #[test]
    fn validate_accepts_valid_input() {
        let tool = PowerShellTool;
        assert!(tool
            .validate_input(&serde_json::json!({ "command": "Write-Output hello" }))
            .is_ok());
    }

    // ── plan-mode gate ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn execute_blocks_dangerous_command_in_read_only_mode() {
        let tool = PowerShellTool;
        let ctx = make_ctx().with_read_only_mode(true);
        let result = tool
            .execute(
                serde_json::json!({ "command": "Remove-Item ./target" }),
                &ctx,
            )
            .await
            .unwrap();
        // Returns a soft error (not an Err), so the LLM sees the reason.
        assert!(!result.success);
        assert!(result.error.as_deref().unwrap_or("").contains("Plan mode"));
    }

    #[tokio::test]
    async fn execute_allows_read_only_command_in_plan_mode() {
        // Only run when PowerShell is actually available
        if PS_EXECUTABLE.is_none() {
            return;
        }
        let tool = PowerShellTool;
        let ctx = make_ctx().with_read_only_mode(true);
        let result = tool
            .execute(serde_json::json!({ "command": "Write-Output hello" }), &ctx)
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.contains("hello"));
    }

    // ── schema ─────────────────────────────────────────────────────────────────

    #[test]
    fn tool_metadata() {
        let tool = PowerShellTool;
        assert_eq!(tool.name(), "powershell");
        assert!(tool.requires_approval());
        let caps = tool.capabilities();
        assert!(caps.contains(&ToolCapability::ExecuteShell));
        assert!(caps.contains(&ToolCapability::SystemModification));
    }
}
