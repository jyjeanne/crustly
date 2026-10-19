//! Tool trait definition

use super::error::Result;
use super::file_read_cache::FileReadCache;
use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

/// A focused task for a sub-agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubAgentTask {
    pub id: Uuid,
    /// Short description (3-5 words), shown in reports and approval prompts.
    pub description: String,
    /// The full task.
    pub prompt: String,
    /// Profile to run it with (`explorer`, `implementer`, `reviewer`,
    /// `general`, or one from `[subagents.profiles]`); `None` = `general`.
    pub profile: Option<String>,
    /// Force read-only, whatever the profile says (the parent is in plan mode).
    pub read_only: bool,
}

/// Where a sub-agent is in its life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubAgentStatus {
    /// Waiting for a free slot (`[subagents] max_concurrent`).
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl SubAgentStatus {
    /// Whether the sub-agent has stopped for good.
    pub fn is_finished(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// What a sub-agent produced, or where it is.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SubAgentReport {
    pub id: Uuid,
    pub description: String,
    pub profile: String,
    pub status: SubAgentStatus,
    /// Its final answer, once completed.
    pub answer: Option<String>,
    /// What went wrong, if it failed.
    pub error: Option<String>,
    /// One-line summary of the tool calls it made.
    pub tools_used: Option<String>,
}

/// Runs sub-agents for the `agent` tool and keeps track of them.
///
/// Implementors must be `Send + Sync` so they can be stored in `Arc` and
/// moved into tokio tasks. The `Debug` bound keeps `ToolExecutionContext`
/// derivable.
#[async_trait]
pub trait SubAgentLauncher: Send + Sync + std::fmt::Debug {
    /// Run `task` and wait for its report. Cancelling `cancel` stops it.
    async fn run(&self, task: SubAgentTask, cancel: CancellationToken) -> SubAgentReport;

    /// Start `task` in the background; collect it later with [`Self::report`].
    async fn start(
        &self,
        task: SubAgentTask,
        cancel: CancellationToken,
    ) -> std::result::Result<(), String>;

    /// The report of a sub-agent started in this session; with `wait`, once
    /// it has finished. `None` for an unknown id.
    async fn report(&self, id: Uuid, wait: bool) -> Option<SubAgentReport>;

    /// Stop a sub-agent. Returns whether it was still running.
    fn cancel(&self, id: Uuid) -> bool;

    /// Every sub-agent started in this session, oldest first.
    fn list(&self) -> Vec<SubAgentReport>;

    /// The profile names sub-agents can be started with.
    fn profiles(&self) -> Vec<String>;
}

/// Execution context for tools
#[derive(Debug, Clone)]
pub struct ToolExecutionContext {
    /// Session ID
    pub session_id: Uuid,

    /// Working directory
    pub working_directory: std::path::PathBuf,

    /// Environment variables
    pub env_vars: HashMap<String, String>,

    /// Whether auto-approve is enabled
    pub auto_approve: bool,

    /// Maximum execution timeout in seconds
    pub timeout_secs: u64,

    /// Whether in read-only mode (Plan mode) - restricts write operations
    pub read_only_mode: bool,

    /// Optional spawner used by the Agent tool to launch sub-agent sessions.
    /// `None` in unit tests or when the caller has not wired up a launcher.
    pub sub_agent_launcher: Option<Arc<dyn SubAgentLauncher>>,

    /// Session-scoped record of which files this session has read (or
    /// written), backing "prior read" enforcement in `edit_file`,
    /// `write_file`'s overwrite path, and `apply_patch`'s Update File
    /// operation - see `file_read_cache` module docs.
    ///
    /// Defaults to a fresh, empty cache scoped to just this context - fine
    /// for tests and other single-call usages. Real sessions should use
    /// [`Self::with_file_read_cache`] to inject the same `Arc` across every
    /// context built for that session, so a read recorded by one tool call
    /// is visible to the next (see `AgentService`).
    pub file_read_cache: Arc<FileReadCache>,

    /// Cancelled when the user cancels the request this call belongs to.
    /// Long-running tools (sub-agents) stop with it.
    pub cancel: CancellationToken,
}

impl ToolExecutionContext {
    /// Create a new execution context
    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            working_directory: std::env::current_dir().unwrap_or_default(),
            env_vars: HashMap::new(),
            auto_approve: false,
            timeout_secs: 30,
            read_only_mode: false,
            sub_agent_launcher: None,
            file_read_cache: Arc::new(FileReadCache::new()),
            cancel: CancellationToken::new(),
        }
    }

    /// Tie this context to the cancellation of the request it belongs to.
    pub fn with_cancel(mut self, cancel: CancellationToken) -> Self {
        self.cancel = cancel;
        self
    }

    /// Set working directory
    pub fn with_working_directory(mut self, dir: std::path::PathBuf) -> Self {
        self.working_directory = dir;
        self
    }

    /// Set auto-approve
    pub fn with_auto_approve(mut self, auto_approve: bool) -> Self {
        self.auto_approve = auto_approve;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Set read-only mode (for Plan mode)
    pub fn with_read_only_mode(mut self, read_only: bool) -> Self {
        self.read_only_mode = read_only;
        self
    }

    /// Set the sub-agent launcher (for Agent tool)
    pub fn with_sub_agent_launcher(mut self, launcher: Arc<dyn SubAgentLauncher>) -> Self {
        self.sub_agent_launcher = Some(launcher);
        self
    }

    /// Inject a session-scoped file-read cache, shared across every context
    /// built for the same session, so reads persist across tool calls (and
    /// across separate `send_message` turns) instead of resetting per call.
    pub fn with_file_read_cache(mut self, cache: Arc<FileReadCache>) -> Self {
        self.file_read_cache = cache;
        self
    }
}

/// Tool result
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// Whether the execution was successful
    pub success: bool,

    /// Output from the tool
    pub output: String,

    /// Error message if unsuccessful
    pub error: Option<String>,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ToolResult {
    /// Create a successful result
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Create an error result
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Tool capability flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCapability {
    /// Can read files
    ReadFiles,
    /// Can write files
    WriteFiles,
    /// Can execute shell commands
    ExecuteShell,
    /// Can access network
    Network,
    /// Can modify system state
    SystemModification,
    /// Can manage plans and tasks
    PlanManagement,
}

/// Tool trait - defines an executable tool
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool name
    fn name(&self) -> &str;

    /// Get the tool description
    fn description(&self) -> &str;

    /// Get the input schema (JSON Schema format)
    fn input_schema(&self) -> Value;

    /// Get the tool's capabilities
    fn capabilities(&self) -> Vec<ToolCapability>;

    /// Check if the tool requires approval before execution
    fn requires_approval(&self) -> bool {
        // By default, dangerous tools require approval
        let dangerous_capabilities = [
            ToolCapability::WriteFiles,
            ToolCapability::ExecuteShell,
            ToolCapability::SystemModification,
        ];

        self.capabilities()
            .iter()
            .any(|cap| dangerous_capabilities.contains(cap))
    }

    /// Execute the tool with given input
    async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>;

    /// Validate input before execution
    fn validate_input(&self, _input: &Value) -> Result<()> {
        // Default implementation - no validation
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_context() {
        let session_id = Uuid::new_v4();
        let ctx = ToolExecutionContext::new(session_id)
            .with_auto_approve(true)
            .with_timeout(60);

        assert_eq!(ctx.session_id, session_id);
        assert!(ctx.auto_approve);
        assert_eq!(ctx.timeout_secs, 60);
    }

    #[test]
    fn test_tool_result_success() {
        let result = ToolResult::success("Done!".to_string())
            .with_metadata("duration_ms".to_string(), "123".to_string());

        assert!(result.success);
        assert_eq!(result.output, "Done!");
        assert!(result.error.is_none());
        assert_eq!(result.metadata.get("duration_ms"), Some(&"123".to_string()));
    }

    #[test]
    fn test_tool_result_error() {
        let result = ToolResult::error("Something went wrong".to_string());

        assert!(!result.success);
        assert_eq!(result.error, Some("Something went wrong".to_string()));
    }
}
