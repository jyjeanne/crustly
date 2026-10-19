//! Agent Service Implementation
//!
//! Core service for managing AI agent conversations, coordinating between
//! LLM providers, context management, and data persistence.

use super::budget::{
    calibration_from, tool_schema_tokens, ContextBudget, ContextUsage, DEFAULT_CONTEXT_WINDOW,
};
use super::compaction;
use super::context::{token_count, AgentContext};
use super::error::{AgentError, Result};
#[cfg(test)]
use super::middleware::tool_call_signature;
use super::middleware::{
    has_mutating_capability, Limits, LoopGuard, Pipeline, ToolCallRecord, Verification,
};
use super::tool_profile::ToolSelection;
use crate::config::ToolProfile;
use crate::llm::provider::router::ModelRouter;
use crate::llm::provider::{
    ContentBlock, ContentDelta, LLMRequest, LLMResponse, Message, PerfMetrics, Provider,
    ProviderStream, StopReason, StreamEvent, TokenUsage, Tool,
};
use crate::llm::tools::cache::{ToolResultCache, ToolTtlConfig};
use crate::llm::tools::{FileReadCache, ToolExecutionContext, ToolRegistry};
use crate::services::{MessageService, ServiceContext, SessionService};
use futures::StreamExt as _;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

mod run;
mod subagents;
mod tool_round;

/// Prefixed to the answer when a response carried no visible text and its
/// reasoning was surfaced as a fallback (see `final_text_and_thinking`). Makes
/// it obvious the body is the model's raw reasoning, not a finished answer.
const REASONING_ONLY_NOTICE: &str =
    "_⚠️ This model returned only its reasoning, not a final answer. Showing the reasoning below._";

/// Outcome of resolving a final response's text (see `final_text_and_thinking`).
///
/// `stored` and `display` differ only in the reasoning-only fallback: `display`
/// carries the `REASONING_ONLY_NOTICE` banner for the UI, while `stored` holds
/// the raw text that is persisted and replayed into model context - the banner
/// must never leak into the conversation history the model sees.
struct FinalText {
    /// What to persist and feed back to the model as prior assistant text.
    stored: String,
    /// What to show the user for this turn (may carry a UI-only notice).
    display: String,
    /// Separate collapsed thinking panel, if any.
    thinking: Option<String>,
}

/// How many times one round is retried after the provider reports that the
/// prompt overflowed its context window, each time after freeing more context.
const MAX_OVERFLOW_RETRIES: usize = 2;

/// The tool schemas of one request, counted once: they do not change while
/// the request runs.
struct RequestTools {
    defs: Vec<Tool>,
    tokens: u32,
}

impl RequestTools {
    fn new(defs: Vec<Tool>) -> Self {
        let tokens = tool_schema_tokens(&defs);
        Self { defs, tokens }
    }
}

/// Gate for `plan complete_task` calls: marking a task `success=true` must be
/// backed by evidence that work actually happened. gemma4 was observed
/// rubber-stamping half a plan - real code for tasks 1-2, then bare
/// `complete_task` calls for tasks 3-4 with zero tool activity in between,
/// leaving a "Completed" plan whose work was never done.
///
/// Returns `Some(rejection message)` when the call must be refused, `None`
/// when it may proceed. A completion is allowed when any of these hold:
/// - any mutating tool (file write, shell, ...) succeeded since the previous
///   task completion in this agent loop (`mutating_evidence > 0`);
/// - the call lists `artifacts` and every listed path exists on disk
///   (verifiable work done earlier or outside this loop);
/// - the task's type is `Research`, which legitimately changes nothing;
/// - the completion is `success=false` (or `skip_task`) - honest failure is
///   always allowed, only unearned success is not.
fn plan_completion_rejection(
    input: &Value,
    mutating_evidence: usize,
    working_directory: &std::path::Path,
    session_id: Uuid,
) -> Option<String> {
    if input.get("operation").and_then(|v| v.as_str()) != Some("complete_task") {
        return None;
    }
    if input.get("success").and_then(|v| v.as_bool()) != Some(true) {
        return None;
    }
    if mutating_evidence > 0 {
        return None;
    }

    // Claimed artifacts count as evidence - but only if they verifiably exist.
    if let Some(artifacts) = input.get("artifacts").and_then(|v| v.as_array()) {
        if !artifacts.is_empty() {
            let missing: Vec<&str> = artifacts
                .iter()
                .filter_map(|a| a.as_str())
                .filter(|p| {
                    let path = std::path::Path::new(p);
                    let abs = if path.is_absolute() {
                        path.to_path_buf()
                    } else {
                        working_directory.join(path)
                    };
                    !abs.exists()
                })
                .collect();
            if missing.is_empty() {
                return None;
            }
            return Some(format!(
                "Refusing to mark this task complete: the claimed artifacts do not exist \
                 on disk: {}. Create them first, correct the paths, or complete with \
                 success=false and explain what went wrong.",
                missing.join(", ")
            ));
        }
    }

    // Research tasks legitimately complete without modifying anything.
    if let Some(order) = input.get("task_order").and_then(|v| v.as_u64()) {
        let plan_file = working_directory.join(format!(".crustly_plan_{}.json", session_id));
        if let Ok(content) = std::fs::read_to_string(&plan_file) {
            if let Ok(plan) = serde_json::from_str::<crate::plan::PlanDocument>(&content) {
                if let Some(task) = plan.tasks.iter().find(|t| t.order == order as usize) {
                    if matches!(task.task_type, crate::plan::TaskType::Research) {
                        return None;
                    }
                }
            }
        }
    }

    Some(
        "Refusing to mark this task complete: no file or shell changes have been observed \
         since the previous task completed, so there is no evidence the task's work was \
         actually done. Either: (1) perform the task's work first, (2) list the files the \
         task produced in `artifacts` so they can be verified on disk, (3) complete with \
         success=false and explain what went wrong, or (4) use skip_task if the task is \
         not applicable."
            .to_string(),
    )
}

/// Returns true for read-only, idempotent tools that can run concurrently.
pub fn is_parallelizable(tool_name: &str) -> bool {
    matches!(
        tool_name,
        "read_file"
            | "glob"
            | "grep"
            | "ls"
            | "web_search"
            | "lsp_hover"
            | "lsp_diagnostics"
            | "lsp_references"
            | "http_get"
    )
}

/// Tool approval request information
#[derive(Debug, Clone)]
pub struct ToolApprovalInfo {
    /// Tool name
    pub tool_name: String,
    /// Tool description
    pub tool_description: String,
    /// Tool input parameters
    pub tool_input: Value,
    /// Tool capabilities
    pub capabilities: Vec<String>,
}

/// Type alias for approval callback function
/// Returns true if approved, false if denied
pub type ApprovalCallback = Arc<
    dyn Fn(ToolApprovalInfo) -> Pin<Box<dyn Future<Output = Result<bool>> + Send>> + Send + Sync,
>;

/// Agent Service for managing AI conversations
pub struct AgentService {
    /// LLM provider
    provider: Arc<dyn Provider>,

    /// Service context for database operations
    context: ServiceContext,

    /// Tool registry for executing tools
    tool_registry: Arc<ToolRegistry>,

    /// Maximum tool execution iterations
    max_tool_iterations: usize,

    /// System prompt template
    default_system_prompt: Option<String>,

    /// Whether to auto-approve tool execution
    auto_approve_tools: bool,

    /// Callback for requesting tool approval from user
    approval_callback: Option<ApprovalCallback>,

    /// Working directory for tool execution
    working_directory: std::path::PathBuf,

    /// Optional model router for tier-based model selection (T046)
    model_router: Option<ModelRouter>,

    /// Session-scoped tool result cache (T037)
    tool_cache: Arc<ToolResultCache>,

    /// Session-scoped file-read cache backing "prior read" enforcement in
    /// edit_file/write_file/apply_patch (see `file_read_cache` module
    /// docs). One `AgentService` instance is one session (matches
    /// `tool_cache`'s treatment above), so a plain shared field - rather
    /// than a map keyed by session_id - is enough for reads to persist
    /// across every `send_message*` call on this instance.
    file_read_cache: Arc<FileReadCache>,

    /// SQLite pool for compaction records (T032). Defaults to the session
    /// database in `context`; set only to write them elsewhere.
    pool: Option<Arc<sqlx::SqlitePool>>,

    /// Whether this service may wire a SubAgentLauncher into tool contexts.
    /// Always false for services created by AgentServiceLauncher to prevent
    /// recursive sub-agent spawning.
    allow_sub_agents: bool,

    /// Which tool schemas requests carry (see `tool_profile`).
    tool_profile: ToolProfile,

    /// Tools exposed on top of the core set when the core profile applies.
    extra_tools: Vec<String>,

    /// Shorter system prompt used while only the core tools are exposed, so
    /// the prompt does not describe tools the model cannot call.
    core_system_prompt: Option<String>,

    /// Provider prompt tokens per estimated token, learned from responses
    /// (see `budget::calibration_from`). Reset when the provider changes.
    token_calibration: std::sync::Mutex<f64>,

    /// Wall-clock cap for one request (`[agent] max_wall_time_secs`).
    max_wall_time: Option<Duration>,

    /// Input + output token cap for one request (`[agent] max_request_tokens`).
    max_request_tokens: Option<u32>,

    /// Whether the model must check its file edits before answering
    /// (`middleware::Verification`).
    verification: bool,

    /// The check command; inferred from the project's build files when unset.
    verification_command: Option<String>,

    /// Whether project notes (AGENTS.md, CLAUDE.md, `.crustly/memory.md`...)
    /// are added to the system prompt (see `project_memory`).
    project_notes: bool,

    /// Whether requests that edited files or ran several tool rounds are
    /// logged to `.crustly/progress.md` (see `project_memory`).
    progress_file: bool,

    /// When set, the only tools the model may use (a sub-agent profile),
    /// overriding `tool_profile`.
    tool_allowlist: Option<Vec<String>>,

    /// Provider a round falls back to when the main one fails or overflows.
    fallback: Option<Fallback>,

    /// The sub-agents started through the `agent` tool, and how many may
    /// run at once.
    subagents: Arc<subagents::SubAgentManager>,

    /// Sub-agent profiles by name (see `profiles`).
    profiles: Arc<std::collections::BTreeMap<String, super::profiles::AgentProfile>>,
}

/// A provider to fall back to (`[fallback]` config).
#[derive(Clone)]
pub struct Fallback {
    pub provider: Arc<dyn Provider>,
    /// Fall back when the main provider is unreachable, rate-limited, times
    /// out or returns a server error.
    pub on_error: bool,
    /// Fall back when the prompt overflows the main model's window and
    /// compaction cannot free enough of it.
    pub on_overflow: bool,
}

/// Route a streaming `TextDelta` through `<think>` tag detection.
///
/// Text outside `<think>` blocks is appended to `text_buf` and forwarded via
/// `chunk_tx` for live TUI rendering. Text inside `<think>` blocks is
/// accumulated in `thinking_buf` and suppressed from `chunk_tx`, so the user
/// never sees raw reasoning tags during streaming.
///
/// `in_think` carries state between calls (tag may span multiple deltas).
fn route_text_delta(
    input: &str,
    in_think: &mut bool,
    text_buf: &mut String,
    thinking_buf: &mut String,
    chunk_tx: Option<&mpsc::UnboundedSender<String>>,
) {
    const OPEN: &str = "<think>";
    const CLOSE: &str = "</think>";

    let mut remaining = input;
    loop {
        if *in_think {
            if let Some(pos) = remaining.find(CLOSE) {
                thinking_buf.push_str(&remaining[..pos]);
                remaining = &remaining[pos + CLOSE.len()..];
                *in_think = false;
            } else {
                // Rest of delta is thinking content.
                thinking_buf.push_str(remaining);
                break;
            }
        } else {
            if let Some(pos) = remaining.find(OPEN) {
                let before = &remaining[..pos];
                if !before.is_empty() {
                    text_buf.push_str(before);
                    if let Some(tx) = chunk_tx {
                        let _ = tx.send(before.to_string());
                    }
                }
                remaining = &remaining[pos + OPEN.len()..];
                *in_think = true;
            } else {
                // No more tags — forward remaining text.
                if !remaining.is_empty() {
                    text_buf.push_str(remaining);
                    if let Some(tx) = chunk_tx {
                        let _ = tx.send(remaining.to_string());
                    }
                }
                break;
            }
        }
    }
}

/// Apply accumulated `input_json_delta` fragments to a streamed tool-use block.
///
/// Anthropic streams tool-call arguments as `InputJsonDelta` fragments *after*
/// the `ContentBlockStart`, which itself carries an empty `input: {}`. Without
/// merging the assembled JSON back in, the tool would be executed with `{}`
/// instead of its real arguments. A no-op for providers (OpenAI, Ollama) that
/// deliver a fully-formed `input` in the `ContentBlockStart` and never emit
/// `InputJsonDelta`.
fn apply_streamed_tool_input(block: ContentBlock, json_buf: &str) -> ContentBlock {
    if let ContentBlock::ToolUse { id, name, input } = block {
        if json_buf.trim().is_empty() {
            return ContentBlock::ToolUse { id, name, input };
        }
        let merged = match serde_json::from_str(json_buf) {
            Ok(value) => value,
            Err(e) => {
                tracing::warn!(
                    "Failed to parse streamed tool arguments for '{}': {} (raw: {})",
                    name,
                    e,
                    json_buf.chars().take(200).collect::<String>()
                );
                input
            }
        };
        ContentBlock::ToolUse {
            id,
            name,
            input: merged,
        }
    } else {
        block
    }
}

/// Consume a [`ProviderStream`] and assemble a complete [`LLMResponse`].
///
/// While consuming the stream, text deltas are optionally forwarded to
/// `chunk_tx` so the TUI can render partial output in real time. Text inside
/// `<think>…</think>` blocks is routed to the thinking buffer and suppressed
/// from `chunk_tx`.
async fn drain_stream_to_response(
    stream: ProviderStream,
    chunk_tx: Option<&mpsc::UnboundedSender<String>>,
    model_name: &str,
) -> crate::llm::provider::Result<LLMResponse> {
    use crate::llm::provider::ProviderError;

    let mut response_id = format!("stream-{}", Uuid::new_v4());
    let mut text_buf = String::new();
    let mut thinking_buf = String::new();
    let mut tool_uses: Vec<ContentBlock> = Vec::new();
    let mut stop_reason: Option<StopReason> = None;
    let mut usage = TokenUsage {
        input_tokens: 0,
        output_tokens: 0,
    };
    let mut perf_metrics: Option<PerfMetrics> = None;

    // A ToolUse block assembled from ContentBlockStart + ContentBlockStop.
    let mut pending_tool: Option<ContentBlock> = None;
    // Accumulates `input_json_delta` fragments for the currently-open tool-use
    // block (Anthropic streams tool arguments this way). Applied on stop.
    let mut pending_tool_json = String::new();

    // Tracks whether we are currently inside a `<think>…</think>` block while
    // routing TextDelta events from tag-based reasoning models (e.g. Ollama
    // DeepSeek-R1, QwQ-32B).
    let mut in_think_block = false;

    futures::pin_mut!(stream);
    while let Some(event_result) = stream.next().await {
        match event_result? {
            StreamEvent::MessageStart { message } => {
                response_id = message.id.clone();
            }
            StreamEvent::ContentBlockStart { content_block, .. } => {
                if matches!(content_block, ContentBlock::ToolUse { .. }) {
                    pending_tool = Some(content_block);
                    pending_tool_json.clear();
                }
            }
            StreamEvent::ContentBlockStop { .. } => {
                if let Some(tool) = pending_tool.take() {
                    tool_uses.push(apply_streamed_tool_input(tool, &pending_tool_json));
                    pending_tool_json.clear();
                }
            }
            StreamEvent::ContentBlockDelta { delta, .. } => match delta {
                ContentDelta::TextDelta { ref text } if !text.is_empty() => {
                    // Route through <think> tag detector: thinking content is
                    // suppressed from chunk_tx, visible text is forwarded.
                    route_text_delta(
                        text,
                        &mut in_think_block,
                        &mut text_buf,
                        &mut thinking_buf,
                        chunk_tx,
                    );
                }
                ContentDelta::ThinkingDelta { ref thinking } if !thinking.is_empty() => {
                    thinking_buf.push_str(thinking);
                }
                // Tool-argument fragments for the open tool-use block
                // (Anthropic). Accumulated and parsed at ContentBlockStop.
                ContentDelta::InputJsonDelta { ref partial_json } if pending_tool.is_some() => {
                    pending_tool_json.push_str(partial_json);
                }
                _ => {}
            },
            StreamEvent::MessageDelta {
                delta,
                usage: u,
                perf_metrics: pm,
            } => {
                stop_reason = delta.stop_reason;
                usage = u;
                perf_metrics = pm;
            }
            StreamEvent::MessageStop => break,
            // Propagate provider-emitted error events as hard errors.
            StreamEvent::Error { error } => {
                return Err(ProviderError::StreamError(error));
            }
            StreamEvent::Ping => {}
        }
    }

    // Flush any tool use block that arrived without a matching ContentBlockStop
    // (e.g. stream truncated or provider omitted the stop event).
    if let Some(tool) = pending_tool.take() {
        tool_uses.push(apply_streamed_tool_input(tool, &pending_tool_json));
    }

    // Fallback: if `route_text_delta` did not extract any thinking (no <think>
    // tags seen during streaming but text contains them — e.g. non-streaming
    // path or partial-tag boundary edge case), extract from the assembled
    // text buffer. This is a no-op when in-stream routing already populated
    // `thinking_buf`.
    if thinking_buf.is_empty() && !text_buf.is_empty() {
        let (tag_thinking, cleaned) = crate::llm::provider::extract_think_tags(&text_buf);
        if !tag_thinking.is_empty() {
            thinking_buf = tag_thinking;
            text_buf = cleaned;
        }
    }

    let mut content: Vec<ContentBlock> = Vec::new();
    if !thinking_buf.is_empty() {
        content.push(ContentBlock::Thinking {
            thinking: thinking_buf,
        });
    }
    if !text_buf.is_empty() {
        content.push(ContentBlock::Text { text: text_buf });
    }
    content.extend(tool_uses);

    Ok(LLMResponse {
        id: response_id,
        model: model_name.to_string(),
        content,
        stop_reason,
        usage,
        cache_metrics: None,
        perf_metrics,
    })
}

impl AgentService {
    /// Create a new agent service
    pub fn new(provider: Arc<dyn Provider>, context: ServiceContext) -> Self {
        Self {
            provider,
            context,
            tool_registry: Arc::new(ToolRegistry::new()),
            max_tool_iterations: 10,
            default_system_prompt: None,
            auto_approve_tools: false,
            approval_callback: None,
            working_directory: std::env::current_dir().unwrap_or_default(),
            model_router: None,
            tool_cache: Arc::new(ToolResultCache::new(ToolTtlConfig::default())),
            file_read_cache: Arc::new(FileReadCache::new()),
            pool: None,
            allow_sub_agents: true,
            tool_profile: ToolProfile::Full,
            extra_tools: Vec::new(),
            core_system_prompt: None,
            token_calibration: std::sync::Mutex::new(1.0),
            max_wall_time: None,
            max_request_tokens: None,
            verification: false,
            verification_command: None,
            project_notes: false,
            progress_file: false,
            tool_allowlist: None,
            fallback: None,
            subagents: Arc::new(subagents::SubAgentManager::new(1)),
            profiles: Arc::new(super::profiles::builtin_profiles()),
        }
    }

    /// Limit the model to exactly these tools (a sub-agent profile).
    pub fn with_tool_allowlist(mut self, tools: Vec<String>) -> Self {
        self.tool_allowlist = Some(tools);
        self
    }

    /// Fall back to another provider when the main one fails or overflows.
    pub fn with_fallback(mut self, fallback: Fallback) -> Self {
        self.fallback = Some(fallback);
        self
    }

    /// Run at most `max_concurrent` sub-agents at a time, with `profiles`.
    pub fn with_subagents(
        mut self,
        max_concurrent: usize,
        profiles: std::collections::BTreeMap<String, super::profiles::AgentProfile>,
    ) -> Self {
        self.subagents = Arc::new(subagents::SubAgentManager::new(max_concurrent));
        self.profiles = Arc::new(profiles);
        self
    }

    /// Cap one request's wall-clock time and tokens. When a cap is reached
    /// the model is asked to answer from the work done so far.
    pub fn with_request_limits(
        mut self,
        max_wall_time: Option<Duration>,
        max_tokens: Option<u32>,
    ) -> Self {
        self.max_wall_time = max_wall_time;
        self.max_request_tokens = max_tokens;
        self
    }

    /// Require the model to check file edits before it answers, with
    /// `command` or one inferred from the project's build files.
    pub fn with_verification(mut self, enabled: bool, command: Option<String>) -> Self {
        self.verification = enabled;
        self.verification_command = command;
        self
    }

    /// Add the project's notes files to the system prompt.
    pub fn with_project_notes(mut self, enabled: bool) -> Self {
        self.project_notes = enabled;
        self
    }

    /// Log finished tool work to `.crustly/progress.md`.
    pub fn with_progress_file(mut self, enabled: bool) -> Self {
        self.progress_file = enabled;
        self
    }

    /// Choose which tool schemas requests carry (the `[tools]` config).
    /// Defaults to every registered tool.
    pub fn with_tool_profile(mut self, profile: ToolProfile, extra: Vec<String>) -> Self {
        self.tool_profile = profile;
        self.extra_tools = extra;
        self
    }

    /// Set the system prompt used while only the core tools are exposed.
    /// Without one, the default system prompt is used for every profile.
    pub fn with_core_system_prompt(mut self, prompt: String) -> Self {
        self.core_system_prompt = Some(prompt);
        self
    }

    /// Enable tier-based model routing (T046)
    pub fn with_model_router(mut self, router: ModelRouter) -> Self {
        self.model_router = Some(router);
        self
    }

    /// Write compaction records to `pool` instead of the session database (T032)
    pub fn with_pool(mut self, pool: Arc<sqlx::SqlitePool>) -> Self {
        self.pool = Some(pool);
        self
    }

    /// Set the default system prompt
    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.default_system_prompt = Some(prompt);
        self
    }

    /// Set maximum tool iterations
    pub fn with_max_tool_iterations(mut self, max: usize) -> Self {
        self.max_tool_iterations = max;
        self
    }

    /// Set the tool registry
    pub fn with_tool_registry(mut self, registry: Arc<ToolRegistry>) -> Self {
        self.tool_registry = registry;
        self
    }

    /// Set whether to auto-approve tool execution
    pub fn with_auto_approve_tools(mut self, auto_approve: bool) -> Self {
        self.auto_approve_tools = auto_approve;
        self
    }

    /// Set the approval callback for interactive tool approval
    pub fn with_approval_callback(mut self, callback: Option<ApprovalCallback>) -> Self {
        self.approval_callback = callback;
        self
    }

    /// Set the working directory for tool execution
    pub fn with_working_directory(mut self, working_directory: std::path::PathBuf) -> Self {
        self.working_directory = working_directory;
        self
    }

    /// The system prompt with the current environment appended.
    ///
    /// The working directory is passed to *tools* via `ToolExecutionContext`, but
    /// was never told to the *model*: the prompt instructs it to "operate on the
    /// current working directory" while never saying what that directory is. A
    /// model asked to "list files in the current folder" therefore has to guess at
    /// its own location - which is exactly when they invent paths (`~/`, `/tmp`) or
    /// fabricate contents outright.
    ///
    /// Appended at send time rather than baked into the `SYSTEM_PROMPT` constant
    /// because the directory is only known at runtime, and a sub-agent may be
    /// running in a different one.
    fn system_prompt_with_env(&self) -> Option<String> {
        self.with_env(self.default_system_prompt.as_deref())
    }

    /// The system prompt for `selection` on a model with `window` tokens: the
    /// core prompt while only the core tools are exposed (when one is set),
    /// the default prompt otherwise, with the environment and - when enabled -
    /// the project notes appended.
    fn system_prompt_for(&self, selection: &ToolSelection, window: u32) -> Option<String> {
        let base = match &self.core_system_prompt {
            Some(core) if selection.is_core_only() => Some(core.as_str()),
            _ => self.default_system_prompt.as_deref(),
        };
        let prompt = self.with_env(base)?;
        if !self.project_notes {
            return Some(prompt);
        }
        let notes = super::project_memory::load_project_notes(
            &self.working_directory,
            super::project_memory::notes_budget(window),
        );
        Some(match notes {
            Some(notes) => format!("{prompt}\n\n{notes}"),
            None => prompt,
        })
    }

    fn with_env(&self, base: Option<&str>) -> Option<String> {
        let base = base?;
        Some(format!(
            "{base}\n\n## Environment\n\nCurrent working directory: {}\n\n\
             This is what \"the current directory/folder\" refers to. Paths you pass to \
             tools are resolved relative to it, so prefer relative paths. Do not assume \
             anything about its contents - list it.",
            self.working_directory.display()
        ))
    }

    /// Control whether this service wires a SubAgentLauncher into tool contexts.
    /// Set to false for sub-agents to prevent recursive spawning.
    pub fn with_allow_sub_agents(mut self, allow: bool) -> Self {
        self.allow_sub_agents = allow;
        self
    }

    /// Swap the active provider in place, keeping every other setting
    /// (tool registry, approval callback, working directory, cache, etc.)
    /// untouched. Used by the TUI's runtime provider-switch dialog, which
    /// must not silently drop the approval callback or tool registry the
    /// way constructing a fresh `AgentService` from scratch would.
    pub fn set_provider(&mut self, provider: Arc<dyn Provider>) {
        self.provider = provider;
        // A different model has a different tokenizer: start over.
        *self
            .token_calibration
            .get_mut()
            .unwrap_or_else(|e| e.into_inner()) = 1.0;
    }

    /// Get the provider name
    pub fn provider_name(&self) -> &str {
        self.provider.name()
    }

    /// Get the default model for this provider
    pub fn provider_model(&self) -> &str {
        self.provider.default_model()
    }

    /// Get the context window (in tokens) for the active provider/model,
    /// if known.
    pub fn provider_context_window(&self) -> Option<u32> {
        self.provider.context_window(self.provider.default_model())
    }

    /// Resolve the tool profile for a model with `context_window` tokens.
    fn tool_selection(&self, context_window: u32) -> ToolSelection {
        match &self.tool_allowlist {
            Some(tools) => ToolSelection::only(tools),
            None => ToolSelection::resolve(self.tool_profile, &self.extra_tools, context_window),
        }
    }

    /// Definitions of the registered tools that `selection` exposes.
    fn exposed_tool_definitions(&self, selection: &ToolSelection) -> Vec<Tool> {
        self.tool_registry
            .get_tool_definitions()
            .into_iter()
            .filter(|t| selection.exposes(&t.name))
            .collect()
    }

    fn default_context_window(&self) -> u32 {
        self.provider_context_window()
            .unwrap_or(DEFAULT_CONTEXT_WINDOW)
    }

    /// Names of the tools the model currently sees, sorted. The TUI uses this
    /// to avoid hinting at tools the core profile does not send.
    pub fn exposed_tool_names(&self) -> Vec<String> {
        let selection = self.tool_selection(self.default_context_window());
        let mut names: Vec<String> = self
            .exposed_tool_definitions(&selection)
            .into_iter()
            .map(|t| t.name)
            .collect();
        names.sort();
        names
    }

    /// Context usage of an empty conversation with the current provider,
    /// model and tool profile: what every request costs before the user says
    /// anything.
    pub fn context_usage_preview(&self) -> ContextUsage {
        let window = self.default_context_window();
        let selection = self.tool_selection(window);
        let tools = self.exposed_tool_definitions(&selection);
        let system = self.system_prompt_for(&selection, window);
        ContextBudget::measure(window, system.as_deref(), &tools, 0, self.calibration())
            .usage(selection.is_core_only())
    }

    fn calibration(&self) -> f64 {
        *self
            .token_calibration
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    /// Learn from the provider's own prompt token count for `budget`'s
    /// request. Skipped when the provider served part of the prompt from its
    /// cache: its input count then leaves the cached tokens out.
    fn record_calibration(&self, response: &LLMResponse, budget: &ContextBudget) {
        if response.cache_metrics.is_some() {
            return;
        }
        if let Some(factor) =
            calibration_from(response.usage.input_tokens, budget.raw_prompt_tokens())
        {
            *self
                .token_calibration
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = factor;
        }
    }

    /// Budget of the request `context` would produce with `tools`.
    fn measure_budget(&self, context: &AgentContext, tools: &RequestTools) -> ContextBudget {
        let system_tokens = context
            .effective_system_prompt()
            .as_deref()
            .map(token_count)
            .unwrap_or(0);
        ContextBudget::from_counts(
            context.max_tokens as u32,
            system_tokens,
            tools.tokens,
            tools.defs.len() as u32,
            context.token_count,
            self.calibration(),
        )
    }

    /// Free context until the answer has room, cheapest step first, and
    /// return whether anything was freed.
    ///
    /// `level` 0 is the check before every round: it acts only when the
    /// budget is tight. Levels 1 and 2 follow a provider overflow error, when
    /// the estimate has already proved too low, and cut deeper each time.
    async fn make_room(
        &self,
        context: &mut AgentContext,
        tools: &RequestTools,
        level: usize,
    ) -> bool {
        if level == 0 && !self.measure_budget(context, tools).is_tight() {
            return false;
        }

        // Step 1: drop the output of older tool calls.
        let keep_results = 2usize.saturating_sub(level);
        let freed = compaction::elide_tool_results(context, keep_results);
        let mut freed_any = freed > 0;
        if freed_any {
            tracing::info!(
                "Elided old tool output, freeing ~{} tokens of context",
                freed
            );
            if level == 0 && !self.measure_budget(context, tools).is_tight() {
                return true;
            }
        }

        // Step 2: summarize and drop older messages. The compaction record
        // goes to the session database, which every service already holds;
        // `with_pool` only overrides it.
        let pool: &sqlx::SqlitePool = self.pool.as_deref().unwrap_or(&self.context.pool);
        let keeps: &[usize] = match level {
            0 => &[compaction::DEFAULT_KEEP_MESSAGES, 4],
            1 => &[4],
            _ => &[2],
        };
        for &keep in keeps {
            match compaction::compact_keeping(context, pool, keep).await {
                Ok(record) => {
                    tracing::info!(
                        "Compacted context: {} -> {} tokens (kept ~{} messages)",
                        record.tokens_before,
                        record.tokens_after,
                        keep
                    );
                    freed_any = true;
                    if !self.measure_budget(context, tools).is_tight() {
                        break;
                    }
                }
                Err(e) => tracing::debug!("Compaction skipped: {}", e),
            }
        }

        let budget = self.measure_budget(context, tools);
        if budget.is_tight() {
            tracing::warn!(
                "Only {} of {} context tokens are left for the answer; the system prompt \
                 and {} tool schemas take {}. Raise the model's context window or set \
                 [tools] profile = \"core\".",
                budget.available(),
                budget.window,
                budget.tool_count,
                budget.fixed_overhead()
            );
        }
        freed_any
    }

    /// The middleware for one request (see `middleware`): limits, loop
    /// guards, and - when enabled, the shell tool is offered and the request
    /// may change files - verification of edits.
    fn round_pipeline(&self, selection: &ToolSelection, read_only: bool) -> Pipeline {
        let mut pipeline = Pipeline::new()
            .with(Limits {
                max_rounds: self.max_tool_iterations,
                max_wall_time: self.max_wall_time,
                max_tokens: self.max_request_tokens,
            })
            .with(LoopGuard::new());
        if self.verification
            && !read_only
            && selection.exposes("bash")
            && self.tool_registry.has_tool("bash")
        {
            if let Some(command) = Verification::detect_command(
                &self.working_directory,
                self.verification_command.as_deref(),
            ) {
                pipeline = pipeline.with(Verification::new(command));
            }
        }
        pipeline
    }

    /// Log a request that edited files or ran several tool rounds to
    /// `.crustly/progress.md`. Failures are logged, not returned: the answer
    /// matters more than the log.
    fn log_progress(
        &self,
        request: &str,
        tools: Option<&str>,
        answer: &str,
        outcome: &run::RunOutcome,
        read_only: bool,
    ) {
        let Some(tools) = tools else {
            return;
        };
        let edited = outcome
            .tool_log
            .iter()
            .any(|r: &ToolCallRecord| r.success && super::middleware::is_file_edit(&r.name));
        if !self.progress_file || read_only || !(edited || outcome.tool_rounds >= 3) {
            return;
        }
        let entry = super::project_memory::ProgressEntry {
            request,
            tools,
            result: answer,
        };
        if let Err(e) = super::project_memory::append_progress(&self.working_directory, &entry) {
            tracing::warn!("Could not update the progress log: {}", e);
        }
    }

    /// Build the provider request for the current context.
    fn build_request(
        model_name: &str,
        context: &AgentContext,
        tools: &RequestTools,
        max_tokens: u32,
    ) -> LLMRequest {
        let mut request = LLMRequest::new(model_name.to_string(), context.messages.clone())
            .with_max_tokens(max_tokens);
        if let Some(system) = context.effective_system_prompt() {
            request = request.with_system(system);
        }
        if !tools.defs.is_empty() {
            request = request.with_tools(tools.defs.clone());
        }
        request
    }

    /// Send a message and get a response
    ///
    /// This will:
    /// 1. Load conversation context from the database
    /// 2. Add the new user message
    /// 3. Send to the LLM provider
    /// 4. Save the response to the database
    /// 5. Update token usage
    pub async fn send_message(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
    ) -> Result<AgentResponse> {
        // Prepare message context (common setup logic)
        let (_model_name, request, message_service, session_service, budget) = self
            .prepare_message_context(session_id, user_message, model)
            .await?;

        // Send to provider
        let response = self
            .provider
            .complete(request)
            .await
            .map_err(AgentError::Provider)?;
        self.record_calibration(&response, &budget);

        // Resolve text: `stored` is persisted/replayed to the model, `display`
        // is shown to the user (they differ only for the reasoning-only notice).
        let final_text = Self::final_text_and_thinking(&response);
        let thinking_text = final_text.thinking;

        // Save assistant response to database (raw text, no UI notice).
        let assistant_db_msg = message_service
            .create_message(session_id, "assistant".to_string(), final_text.stored)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Calculate total tokens and cost for this message
        let total_tokens = response.usage.input_tokens + response.usage.output_tokens;
        let cost = self.provider.calculate_cost(
            &response.model,
            response.usage.input_tokens,
            response.usage.output_tokens,
        );

        // Update message with usage info
        message_service
            .update_message_usage(assistant_db_msg.id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Record which provider answered and, if available, its perf metrics
        // (load/prefill/generation durations) - currently only Ollama.
        message_service
            .update_message_metrics(
                assistant_db_msg.id,
                self.provider.name(),
                response.perf_metrics.as_ref(),
            )
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Update session token usage
        session_service
            .update_session_usage(session_id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        Ok(AgentResponse {
            message_id: assistant_db_msg.id,
            content: final_text.display,
            thinking_text,
            stop_reason: response.stop_reason,
            usage: response.usage,
            cost,
            model: response.model,
            provider_name: self.provider.name().to_string(),
            perf_metrics: response.perf_metrics,
            context: Some(budget.usage(false)),
            tool_summary: None,
        })
    }

    /// Send a message and get a streaming response
    ///
    /// Returns a stream of response chunks that can be consumed incrementally.
    pub async fn send_message_streaming(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
    ) -> Result<AgentStreamResponse> {
        // Prepare message context (common setup logic)
        let (model_name, request, _message_service, _session_service, _budget) = self
            .prepare_message_context(session_id, user_message, model)
            .await?;

        // Add streaming flag to request
        let request = request.with_streaming();

        // Get streaming response from provider
        let stream = self
            .provider
            .stream(request)
            .await
            .map_err(AgentError::Provider)?;

        Ok(AgentStreamResponse {
            session_id,
            message_id: Uuid::new_v4(),
            stream,
            model: model_name,
        })
    }

    /// Send a message with automatic tool execution
    ///
    /// This method implements a tool execution loop:
    /// 1. Send message to LLM
    /// 2. If LLM requests tool use, execute the tool
    /// 3. Send tool results back to LLM
    /// 4. Repeat until LLM finishes or max iterations reached
    pub async fn send_message_with_tools(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
    ) -> Result<AgentResponse> {
        self.send_message_with_tools_and_mode(session_id, user_message, model, false)
            .await
    }

    /// Send a message with automatic tool execution and explicit read-only mode control.
    pub async fn send_message_with_tools_and_mode(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
        read_only_mode: bool,
    ) -> Result<AgentResponse> {
        self.send_message_with_tools_inner(
            session_id,
            user_message,
            model,
            read_only_mode,
            None,
            CancellationToken::new(),
        )
        .await
    }

    /// Non-streaming variant that stops with [`AgentError::Cancelled`] when
    /// `cancel` is cancelled (sub-agents).
    pub(crate) async fn send_message_with_tools_cancellable(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
        read_only_mode: bool,
        cancel: CancellationToken,
    ) -> Result<AgentResponse> {
        self.send_message_with_tools_inner(
            session_id,
            user_message,
            model,
            read_only_mode,
            None,
            cancel,
        )
        .await
    }

    /// What a sub-agent of this service inherits from it.
    fn child_template(&self) -> subagents::ChildTemplate {
        subagents::ChildTemplate {
            provider: self.provider.clone(),
            fallback: self.fallback.clone(),
            context: self.context.clone(),
            tool_registry: self.tool_registry.clone(),
            working_directory: self.working_directory.clone(),
            system_prompt: self.default_system_prompt.clone(),
            core_system_prompt: self.core_system_prompt.clone(),
            tool_profile: self.tool_profile,
            extra_tools: self.extra_tools.clone(),
            approval_callback: self.approval_callback.clone(),
            auto_approve_tools: self.auto_approve_tools,
            verification: self.verification,
            verification_command: self.verification_command.clone(),
            project_notes: self.project_notes,
            profiles: self.profiles.clone(),
        }
    }

    /// Streaming variant — identical to [`send_message_with_tools_and_mode`] but forwards
    /// each LLM text delta to `chunk_tx` so the TUI can render incremental output.
    /// Cancelling `cancel` ends the request with [`AgentError::Cancelled`] at
    /// the next model call, approval prompt or round boundary.
    pub async fn send_message_with_tools_and_mode_streaming(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
        read_only_mode: bool,
        chunk_tx: mpsc::UnboundedSender<String>,
        cancel: CancellationToken,
    ) -> Result<AgentResponse> {
        self.send_message_with_tools_inner(
            session_id,
            user_message,
            model,
            read_only_mode,
            Some(chunk_tx),
            cancel,
        )
        .await
    }

    /// Core implementation — `chunk_tx` is `Some` when streaming to TUI, `None` for CLI/tests.
    async fn send_message_with_tools_inner(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
        read_only_mode: bool,
        chunk_tx: Option<mpsc::UnboundedSender<String>>,
        cancel: CancellationToken,
    ) -> Result<AgentResponse> {
        // Get or create session
        let session_service = SessionService::new(self.context.clone());
        let _session = session_service
            .get_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?
            .ok_or(AgentError::SessionNotFound(session_id))?;

        // Load conversation context
        let message_service = MessageService::new(self.context.clone());
        let db_messages = message_service
            .list_messages_for_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Resolve the model first, so the context window and tool profile
        // below are those of the model that will answer. An explicit model
        // (a sub-agent profile) wins; otherwise the router, when enabled,
        // picks the tier's model from the request's wording (T046).
        let model_name = match (model, &self.model_router) {
            (Some(model), _) => model,
            (None, Some(router)) => {
                let tier =
                    crate::tui::prompt_analyzer::PromptAnalyzer::new().classify_tier(&user_message);
                let (_, model_id) = router.resolve(tier);
                tracing::info!("Model router: {:?} request -> {}", tier, model_id);
                model_id.to_string()
            }
            (None, None) => self.provider.default_model().to_string(),
        };
        let context_window = self
            .provider
            .context_window(&model_name)
            .unwrap_or(DEFAULT_CONTEXT_WINDOW);

        let mut context =
            AgentContext::from_db_messages(session_id, db_messages, context_window as usize);

        // Which tools the model sees on this window, counted once: the
        // schemas ride along with every round of the loop.
        let selection = self.tool_selection(context_window);
        let tools = RequestTools::new(self.exposed_tool_definitions(&selection));
        if tools.defs.is_empty() {
            tracing::warn!("No tools registered in tool registry!");
        }

        // Add system prompt if available, with the working directory appended so
        // the model knows where "the current directory" actually is.
        if let Some(system_prompt) = self.system_prompt_for(&selection, context_window) {
            context.system_prompt = Some(system_prompt);
        }

        // What the user asked, for the progress log.
        let request_line = user_message
            .trim()
            .lines()
            .next()
            .unwrap_or_default()
            .to_string();

        // Auto-inject PDF content when the user message references a .pdf file
        let user_message = crate::llm::pdf_context::augment_message_with_pdf(
            &user_message,
            &self.working_directory,
        )
        .await;

        // Add user message
        let user_msg = Message::user(user_message.clone());
        context.add_message(user_msg);

        // Save user message to database
        let _user_db_msg = message_service
            .create_message(session_id, "user".to_string(), user_message)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Create tool execution context.
        // Only wire a SubAgentLauncher for top-level services; sub-agents are
        // built with allow_sub_agents=false to prevent recursive spawning.
        let mut tool_context = ToolExecutionContext::new(session_id)
            .with_auto_approve(self.auto_approve_tools)
            .with_working_directory(self.working_directory.clone())
            .with_read_only_mode(read_only_mode)
            .with_file_read_cache(self.file_read_cache.clone())
            .with_cancel(cancel.clone());
        if self.allow_sub_agents {
            let launcher = Arc::new(subagents::AgentServiceLauncher::new(
                self.subagents.clone(),
                self.child_template(),
            ));
            tool_context = tool_context.with_sub_agent_launcher(launcher);
        }

        // Run rounds of model and tool calls until the model answers (see
        // `run`); the middleware decides when to stop early.
        let pipeline = self.round_pipeline(&selection, read_only_mode);
        let outcome = run::AgentRun {
            service: self,
            provider: self.provider.clone(),
            model_name,
            context,
            selection: &selection,
            tools: &tools,
            tool_context,
            chunk_tx: chunk_tx.as_ref(),
            cancel: &cancel,
            pipeline,
        }
        .run()
        .await?;
        let response = &outcome.response;

        // Resolve text: `stored` is persisted/replayed to the model, `display`
        // is shown to the user (they differ only by UI notices).
        let final_text = Self::final_text_and_thinking(response);
        let thinking_text = final_text.thinking;
        let display = match &outcome.notice {
            Some(notice) => format!("{}\n\n{}", final_text.display, notice),
            None => final_text.display,
        };

        // Save final assistant response to database (raw text, no UI notice).
        let assistant_db_msg = message_service
            .create_message(
                session_id,
                "assistant".to_string(),
                final_text.stored.clone(),
            )
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Tool calls and their results are not stored, so record which tools
        // produced this answer: a later request then knows what this one
        // read, changed or ran (see `AgentContext::from_db_messages`).
        let tool_summary = super::project_memory::summarize_tool_calls(&outcome.tool_log);
        if let Some(summary) = &tool_summary {
            message_service
                .update_message_tool_log(assistant_db_msg.id, summary)
                .await
                .map_err(|e| AgentError::Database(e.to_string()))?;
        }
        self.log_progress(
            &request_line,
            tool_summary.as_deref(),
            &final_text.stored,
            &outcome,
            read_only_mode,
        );

        // Each model call was priced by the provider that made it (the run
        // may have fallen back part-way).
        let total_input_tokens = outcome.input_tokens;
        let total_output_tokens = outcome.output_tokens;
        let total_tokens = total_input_tokens + total_output_tokens;
        let cost = outcome.cost;

        // Update message with usage info
        message_service
            .update_message_usage(assistant_db_msg.id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Record which provider answered and, if available, its perf metrics.
        message_service
            .update_message_metrics(
                assistant_db_msg.id,
                &outcome.provider_name,
                response.perf_metrics.as_ref(),
            )
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Update session token usage
        session_service
            .update_session_usage(session_id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        Ok(AgentResponse {
            message_id: assistant_db_msg.id,
            content: display,
            thinking_text,
            stop_reason: response.stop_reason.clone(),
            usage: crate::llm::provider::TokenUsage {
                input_tokens: total_input_tokens,
                output_tokens: total_output_tokens,
            },
            cost,
            model: response.model.clone(),
            provider_name: outcome.provider_name.clone(),
            perf_metrics: response.perf_metrics.clone(),
            context: outcome
                .last_budget
                .map(|b| b.usage(selection.is_core_only())),
            tool_summary,
        })
    }

    /// Call the provider for one tool-loop iteration.
    ///
    /// Streams when `chunk_tx` is `Some`, forwarding text deltas to the sender.
    /// Falls back to a blocking `complete()` call when `None` (CLI / tests).
    async fn call_provider_streaming(
        provider: &Arc<dyn Provider>,
        request: LLMRequest,
        chunk_tx: Option<&mpsc::UnboundedSender<String>>,
        model_name: &str,
    ) -> crate::llm::provider::Result<LLMResponse> {
        if let Some(tx) = chunk_tx {
            let stream = provider.stream(request.with_streaming()).await?;
            drain_stream_to_response(stream, Some(tx), model_name).await
        } else {
            provider.complete(request).await
        }
    }

    /// Helper to prepare message context for LLM requests
    ///
    /// This extracts the common setup logic shared between send_message() and
    /// send_message_streaming() to reduce code duplication.
    async fn prepare_message_context(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
    ) -> Result<(
        String,
        LLMRequest,
        MessageService,
        SessionService,
        ContextBudget,
    )> {
        // Get or create session
        let session_service = SessionService::new(self.context.clone());
        let _session = session_service
            .get_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?
            .ok_or(AgentError::SessionNotFound(session_id))?;

        // Load conversation context
        let message_service = MessageService::new(self.context.clone());
        let db_messages = message_service
            .list_messages_for_session(session_id)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        let model_name = model.unwrap_or_else(|| self.provider.default_model().to_string());
        let context_window = self
            .provider
            .context_window(&model_name)
            .unwrap_or(DEFAULT_CONTEXT_WINDOW);

        let mut context =
            AgentContext::from_db_messages(session_id, db_messages, context_window as usize);

        // Add system prompt if available, with the working directory appended so
        // the model knows where "the current directory" actually is.
        if let Some(system_prompt) = self.system_prompt_with_env() {
            context.system_prompt = Some(system_prompt);
        }

        // Auto-inject PDF content when the user message references a .pdf file
        let user_message = crate::llm::pdf_context::augment_message_with_pdf(
            &user_message,
            &self.working_directory,
        )
        .await;

        // Add user message
        let user_msg = Message::user(user_message.clone());
        context.add_message(user_msg);

        // Save user message to database
        message_service
            .create_message(session_id, "user".to_string(), user_message)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Build base LLM request with a budget that fits the configured
        // provider context window after accounting for the system prompt.
        let no_tools = RequestTools::new(Vec::new());
        let budget = self.measure_budget(&context, &no_tools);
        let request =
            Self::build_request(&model_name, &context, &no_tools, budget.completion_budget());

        Ok((
            model_name,
            request,
            message_service,
            session_service,
            budget,
        ))
    }

    /// Extract text content from an LLM response.
    fn extract_text_from_response(response: &LLMResponse) -> String {
        let mut text = String::new();

        for content in &response.content {
            match content {
                ContentBlock::Text { text: t } => {
                    text.push_str(t);
                }
                ContentBlock::ToolUse { name, input, .. } => {
                    // Format tool use for display
                    text.push_str(&format!("\n[Tool: {}]\n{}\n", name, input));
                }
                _ => {}
            }
        }

        text
    }

    /// Resolve a *final* response into what to store vs. what to display.
    ///
    /// Normally `stored` and `display` are identical (the visible text), with
    /// `thinking` kept separate for the collapsed panel. But some reasoning
    /// models served through Ollama (ornith, DeepSeek-R1, QwQ) put their
    /// *entire* final answer in the reasoning channel and emit an empty
    /// `content` on the closing turn. Left alone, that saved a blank assistant
    /// message - the TUI showed only a collapsed "[Thinking ▸]" toggle with
    /// nothing under it.
    ///
    /// When there is no visible text but there IS thinking, promote the
    /// thinking to the answer (and drop the separate thinking panel, so the
    /// same text is not rendered twice). Crucially, only `display` gets the
    /// `REASONING_ONLY_NOTICE` heads-up; `stored` holds the RAW reasoning. The
    /// stored value is what gets persisted and later replayed into the model's
    /// context as prior assistant text - putting a UI banner there would make
    /// the model think it authored the notice. When a real text block is
    /// present, this promotion never fires and stored == display.
    fn final_text_and_thinking(response: &LLMResponse) -> FinalText {
        let text = Self::extract_text_from_response(response);
        let thinking = Self::extract_thinking_from_response(response);

        if text.trim().is_empty() {
            if let Some(thinking) = thinking {
                return FinalText {
                    display: format!("{}\n\n{}", REASONING_ONLY_NOTICE, thinking),
                    stored: thinking,
                    thinking: None,
                };
            }
        }

        FinalText {
            display: text.clone(),
            stored: text,
            thinking,
        }
    }

    /// Extract thinking content from an LLM response (extended thinking blocks only).
    fn extract_thinking_from_response(response: &LLMResponse) -> Option<String> {
        let mut thinking = String::new();
        for content in &response.content {
            if let ContentBlock::Thinking { thinking: t } = content {
                if !thinking.is_empty() {
                    thinking.push('\n');
                }
                thinking.push_str(t);
            }
        }
        if thinking.is_empty() {
            None
        } else {
            Some(thinking)
        }
    }
}

/// Response from the agent
#[derive(Debug, Clone)]
pub struct AgentResponse {
    /// Message ID in database
    pub message_id: Uuid,

    /// Response content
    pub content: String,

    /// Extended thinking text (Anthropic claude-3-7-sonnet+ only), if present
    pub thinking_text: Option<String>,

    /// Stop reason
    pub stop_reason: Option<StopReason>,

    /// Token usage
    pub usage: crate::llm::provider::TokenUsage,

    /// Cost in USD
    pub cost: f64,

    /// Model used
    pub model: String,

    /// Name of the provider that served this response (e.g. "ollama",
    /// "openai", "anthropic"). Lets the TUI show which backend answered.
    pub provider_name: String,

    /// Runtime performance metrics, if the provider exposes them
    /// (currently only the native Ollama provider).
    pub perf_metrics: Option<crate::llm::provider::PerfMetrics>,

    /// How full the context window was for the last request of this turn.
    pub context: Option<ContextUsage>,

    /// One-line summary of the tool calls behind this answer, if any.
    pub tool_summary: Option<String>,
}

/// Streaming response from the agent
pub struct AgentStreamResponse {
    /// Session ID
    pub session_id: Uuid,

    /// Message ID that will be created
    pub message_id: Uuid,

    /// Stream of events
    pub stream: ProviderStream,

    /// Model being used
    pub model: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::llm::provider::{LLMRequest, LLMResponse, TokenUsage};
    use crate::llm::tools::ToolCapability;
    use async_trait::async_trait;

    /// Mock provider for testing
    struct MockProvider;

    #[async_trait]
    impl Provider for MockProvider {
        async fn complete(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<LLMResponse> {
            Ok(LLMResponse {
                id: "test-response-1".to_string(),
                model: "mock-model".to_string(),
                content: vec![ContentBlock::Text {
                    text: "This is a test response".to_string(),
                }],
                stop_reason: Some(StopReason::EndTurn),
                usage: TokenUsage {
                    input_tokens: 10,
                    output_tokens: 20,
                },
                cache_metrics: None,
                perf_metrics: None,
            })
        }

        async fn stream(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<ProviderStream> {
            unimplemented!("Streaming not needed for basic tests")
        }

        fn name(&self) -> &str {
            "mock"
        }

        fn default_model(&self) -> &str {
            "mock-model"
        }

        fn supported_models(&self) -> Vec<String> {
            vec!["mock-model".to_string()]
        }

        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(4096)
        }

        fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
            0.001 // Mock cost
        }
    }

    /// Regression: the loop-detection signature for edit_file/write_file/
    /// read_file read the `file_path` key, but the tools' canonical input key
    /// is `path` (file_path is only an alias). So a real call using `path`
    /// produced a bare `edit_file:` signature; two edits to *different* paths
    /// then collided and the loop detector aborted the second before it ran -
    /// which is exactly why an approved `edit_file ./exercice1` never executed.
    #[test]
    fn signature_uses_path_key_so_different_edits_do_not_collide() {
        let a = tool_call_signature(
            "edit_file",
            &serde_json::json!({ "path": "./exercice1", "operation": "create" }),
        );
        let b = tool_call_signature(
            "edit_file",
            &serde_json::json!({ "path": "./exercice2", "operation": "create" }),
        );
        assert_eq!(a, "edit_file:./exercice1");
        assert_ne!(a, b, "different paths must yield different signatures");
    }

    #[test]
    fn signature_accepts_file_path_alias() {
        // The alias must still work and agree with the canonical key.
        let via_alias =
            tool_call_signature("read_file", &serde_json::json!({ "file_path": "src/x.rs" }));
        let via_path = tool_call_signature("read_file", &serde_json::json!({ "path": "src/x.rs" }));
        assert_eq!(via_alias, "read_file:src/x.rs");
        assert_eq!(via_alias, via_path);
    }

    #[test]
    fn signature_distinguishes_same_tool_different_args() {
        // A quick sweep across the tools with dedicated signature rules.
        let sig = tool_call_signature;
        assert_ne!(
            sig("write_file", &serde_json::json!({ "path": "a" })),
            sig("write_file", &serde_json::json!({ "path": "b" }))
        );
        assert_ne!(
            sig("bash", &serde_json::json!({ "command": "mkdir a" })),
            sig("bash", &serde_json::json!({ "command": "mkdir b" }))
        );
        assert_ne!(
            sig("ls", &serde_json::json!({ "path": "src" })),
            sig("ls", &serde_json::json!({ "path": "tests" }))
        );
    }

    /// Regression: gemma4 rubber-stamped plan tasks 3-4 - bare
    /// `complete_task(success=true)` calls with zero tool activity in
    /// between - leaving a "Completed" plan whose work was never done.
    /// Unearned success must be refused; every honest path must pass.
    #[test]
    fn plan_completion_gate_decision_matrix() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path();
        let session = Uuid::new_v4();
        let complete_ok = serde_json::json!({
            "operation": "complete_task", "task_order": 2, "success": true, "output": "done"
        });

        // The observed failure: success=true, no evidence, no artifacts.
        let rejection = plan_completion_rejection(&complete_ok, 0, dir, session);
        assert!(rejection.is_some(), "unearned success=true must be refused");
        assert!(
            rejection.unwrap().contains("skip_task"),
            "the refusal must teach the honest ways out"
        );

        // Evidence of work since the last completion -> allowed.
        assert!(plan_completion_rejection(&complete_ok, 1, dir, session).is_none());

        // Honest failure and unrelated plan operations are never gated.
        let failed = serde_json::json!({
            "operation": "complete_task", "task_order": 2, "success": false, "output": "broke"
        });
        assert!(plan_completion_rejection(&failed, 0, dir, session).is_none());
        let add = serde_json::json!({ "operation": "add_task", "title": "T" });
        assert!(plan_completion_rejection(&add, 0, dir, session).is_none());

        // Artifacts that exist on disk are verifiable evidence...
        std::fs::write(dir.join("made.py"), "x").unwrap();
        let with_real_artifact = serde_json::json!({
            "operation": "complete_task", "task_order": 2, "success": true,
            "output": "done", "artifacts": ["made.py"]
        });
        assert!(plan_completion_rejection(&with_real_artifact, 0, dir, session).is_none());

        // ...but claimed artifacts that DON'T exist are a firm refusal.
        let with_fake_artifact = serde_json::json!({
            "operation": "complete_task", "task_order": 2, "success": true,
            "output": "done", "artifacts": ["made.py", "ghost.py"]
        });
        let rejection = plan_completion_rejection(&with_fake_artifact, 0, dir, session);
        assert!(
            rejection.as_deref().is_some_and(|m| m.contains("ghost.py")),
            "missing artifacts must be named, got: {rejection:?}"
        );

        // A Research task legitimately changes nothing - exempt via the plan
        // file's task type.
        let mut plan =
            crate::plan::PlanDocument::new(session, "Investigate".to_string(), String::new());
        plan.tasks.push(crate::plan::PlanTask::new(
            2,
            "Read the codebase".to_string(),
            String::new(),
            crate::plan::TaskType::Research,
        ));
        std::fs::write(
            dir.join(format!(".crustly_plan_{session}.json")),
            serde_json::to_string(&plan).unwrap(),
        )
        .unwrap();
        assert!(
            plan_completion_rejection(&complete_ok, 0, dir, session).is_none(),
            "a Research task must complete without mutating evidence"
        );
    }

    fn response_with(content: Vec<ContentBlock>) -> LLMResponse {
        LLMResponse {
            id: "r".to_string(),
            model: "m".to_string(),
            content,
            stop_reason: Some(StopReason::EndTurn),
            usage: TokenUsage {
                input_tokens: 1,
                output_tokens: 1,
            },
            cache_metrics: None,
            perf_metrics: None,
        }
    }

    /// Regression: a reasoning model (ornith, DeepSeek-R1, QwQ via Ollama)
    /// answered entirely inside its thinking channel and returned an empty
    /// `content` on the final turn. That saved a blank assistant message - the
    /// TUI showed only a collapsed "[Thinking ▸]" toggle with no answer under
    /// it. The thinking must be promoted to the visible answer, AND removed
    /// from the separate thinking panel so the same text is not shown twice.
    ///
    /// The `display` value carries the reasoning-only notice for the user, but
    /// `stored` (persisted and replayed into model context) must be the RAW
    /// reasoning with NO notice - otherwise the model sees the UI banner as its
    /// own prior words on the next turn.
    #[test]
    fn final_text_falls_back_to_thinking_when_there_is_no_visible_text() {
        let reasoning = "The directory contains src/, README.md and main.rs.";
        let response = response_with(vec![ContentBlock::Thinking {
            thinking: reasoning.to_string(),
        }]);
        let ft = AgentService::final_text_and_thinking(&response);

        assert!(
            ft.display.starts_with(REASONING_ONLY_NOTICE),
            "the display value must carry the heads-up, got: {}",
            ft.display
        );
        assert!(
            ft.display.contains(reasoning),
            "the promoted reasoning must be present in the display value"
        );

        assert_eq!(
            ft.stored, reasoning,
            "the STORED value must be the raw reasoning with no UI notice, \
             so the banner never leaks into model-facing history"
        );
        assert!(
            !ft.stored.contains(REASONING_ONLY_NOTICE),
            "the notice must not be persisted"
        );
        assert!(
            ft.thinking.is_none(),
            "promoted thinking must not also render in the thinking panel"
        );
    }

    /// The promotion must NOT fire when a real text block is present: the
    /// visible answer is used and thinking stays separate (collapsed).
    #[test]
    fn final_text_prefers_visible_text_and_keeps_thinking_separate() {
        let response = response_with(vec![
            ContentBlock::Thinking {
                thinking: "internal reasoning".to_string(),
            },
            ContentBlock::Text {
                text: "Here is the answer.".to_string(),
            },
        ]);
        let ft = AgentService::final_text_and_thinking(&response);
        assert_eq!(ft.display, "Here is the answer.");
        assert_eq!(
            ft.stored, "Here is the answer.",
            "with a real text block, stored == display"
        );
        assert_eq!(ft.thinking.as_deref(), Some("internal reasoning"));
    }

    /// A genuinely empty response (no text, no thinking) stays empty - the
    /// fallback has nothing to offer and must not invent content.
    #[test]
    fn final_text_of_an_empty_response_is_empty() {
        let response = response_with(vec![]);
        let ft = AgentService::final_text_and_thinking(&response);
        assert_eq!(ft.display, "");
        assert_eq!(ft.stored, "");
        assert!(ft.thinking.is_none());
    }

    async fn create_test_service() -> (AgentService, Uuid) {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();
        let pool = db.pool().clone();

        let context = ServiceContext::new(pool);
        let provider = Arc::new(MockProvider);

        let agent_service = AgentService::new(provider, context.clone());

        // Create a test session
        let session_service = SessionService::new(context);
        let session = session_service
            .create_session(Some("Test Session".to_string()))
            .await
            .unwrap();

        (agent_service, session.id)
    }

    #[tokio::test]
    async fn test_agent_service_creation() {
        let (agent_service, _) = create_test_service().await;
        assert_eq!(agent_service.max_tool_iterations, 10);
    }

    #[tokio::test]
    async fn test_send_message() {
        let (agent_service, session_id) = create_test_service().await;

        let response = agent_service
            .send_message(session_id, "Hello, world!".to_string(), None)
            .await
            .unwrap();

        assert!(!response.content.is_empty());
        assert_eq!(response.model, "mock-model");
        assert!(response.cost > 0.0);
    }

    #[tokio::test]
    async fn test_send_message_with_system_prompt() {
        let (agent_service, session_id) = create_test_service().await;

        let agent_service =
            agent_service.with_system_prompt("You are a helpful assistant.".to_string());

        let response = agent_service
            .send_message(session_id, "Hello!".to_string(), None)
            .await
            .unwrap();

        assert!(!response.content.is_empty());
    }

    /// Regression: the working directory was threaded into `ToolExecutionContext`
    /// but never into the system prompt, so the prompt told the model to "operate
    /// on the current working directory" without ever saying which one that was.
    /// Asked to list "the current folder", the model had to guess where it was -
    /// and guessing is when they invent `~/` paths or fabricate contents.
    #[tokio::test]
    async fn system_prompt_tells_the_model_the_working_directory() {
        let (agent_service, _session_id) = create_test_service().await;
        let workdir = std::env::temp_dir().join("crustly-cwd-test");

        let agent_service = agent_service
            .with_system_prompt("You are a helpful assistant.".to_string())
            .with_working_directory(workdir.clone());

        let prompt = agent_service
            .system_prompt_with_env()
            .expect("a system prompt was set");

        assert!(
            prompt.contains(&workdir.display().to_string()),
            "the model must be told its working directory; prompt was:\n{prompt}"
        );
        assert!(
            prompt.contains("You are a helpful assistant."),
            "the base prompt must be preserved, not replaced"
        );
    }

    /// With no system prompt configured there is nothing to append to, and the
    /// environment block must not conjure one into existence.
    #[tokio::test]
    async fn system_prompt_with_env_is_none_when_no_prompt_is_set() {
        let (agent_service, _session_id) = create_test_service().await;
        assert!(agent_service.system_prompt_with_env().is_none());
    }

    // Sub-agent approval: see `sub_agents_without_an_approval_prompt_cannot_run_gated_tools`
    // and `sub_agent_tool_calls_are_approved_through_the_parent_s_prompt`.

    /// Mock provider that simulates tool use
    struct MockProviderWithTools {
        call_count: std::sync::Mutex<usize>,
    }

    impl MockProviderWithTools {
        fn new() -> Self {
            Self {
                call_count: std::sync::Mutex::new(0),
            }
        }
    }

    #[async_trait]
    impl Provider for MockProviderWithTools {
        async fn complete(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<LLMResponse> {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;
            let call_num = *count;

            if call_num == 1 {
                // First call: request tool use
                Ok(LLMResponse {
                    id: "test-response-1".to_string(),
                    model: "mock-model".to_string(),
                    content: vec![
                        ContentBlock::Text {
                            text: "I'll use the test tool.".to_string(),
                        },
                        ContentBlock::ToolUse {
                            id: "tool-1".to_string(),
                            name: "test_tool".to_string(),
                            input: serde_json::json!({"message": "test"}),
                        },
                    ],
                    stop_reason: Some(StopReason::ToolUse),
                    usage: TokenUsage {
                        input_tokens: 10,
                        output_tokens: 20,
                    },
                    cache_metrics: None,
                    perf_metrics: None,
                })
            } else {
                // Second call: final response after tool execution
                Ok(LLMResponse {
                    id: "test-response-2".to_string(),
                    model: "mock-model".to_string(),
                    content: vec![ContentBlock::Text {
                        text: "Tool execution completed successfully.".to_string(),
                    }],
                    stop_reason: Some(StopReason::EndTurn),
                    usage: TokenUsage {
                        input_tokens: 15,
                        output_tokens: 25,
                    },
                    cache_metrics: None,
                    perf_metrics: None,
                })
            }
        }

        async fn stream(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<ProviderStream> {
            unimplemented!("Streaming not needed for tool tests")
        }

        fn name(&self) -> &str {
            "mock-with-tools"
        }

        fn default_model(&self) -> &str {
            "mock-model"
        }

        fn supported_models(&self) -> Vec<String> {
            vec!["mock-model".to_string()]
        }

        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(4096)
        }

        fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
            0.001
        }
    }

    /// Mock tool for testing
    struct MockTool;

    #[async_trait]
    impl crate::llm::tools::Tool for MockTool {
        fn name(&self) -> &str {
            "test_tool"
        }

        fn description(&self) -> &str {
            "A test tool"
        }

        fn input_schema(&self) -> serde_json::Value {
            serde_json::json!({
                "type": "object",
                "properties": {
                    "message": {"type": "string"}
                }
            })
        }

        fn capabilities(&self) -> Vec<crate::llm::tools::ToolCapability> {
            vec![]
        }

        fn requires_approval(&self) -> bool {
            false
        }

        async fn execute(
            &self,
            _input: serde_json::Value,
            _context: &crate::llm::tools::ToolExecutionContext,
        ) -> crate::llm::tools::Result<crate::llm::tools::ToolResult> {
            Ok(crate::llm::tools::ToolResult::success(
                "Tool executed successfully".to_string(),
            ))
        }
    }

    #[tokio::test]
    async fn test_send_message_with_tool_execution() {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();
        let pool = db.pool().clone();

        let context = ServiceContext::new(pool);
        let provider = Arc::new(MockProviderWithTools::new());

        // Create tool registry and register our test tool
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool));

        let agent_service = AgentService::new(provider, context.clone())
            .with_tool_registry(Arc::new(registry))
            .with_auto_approve_tools(true);

        // Create a test session
        let session_service = SessionService::new(context);
        let session = session_service
            .create_session(Some("Test Session".to_string()))
            .await
            .unwrap();

        // Send message with tool execution
        let response = agent_service
            .send_message_with_tools(session.id, "Use the test tool".to_string(), None)
            .await
            .unwrap();

        assert!(!response.content.is_empty());
        assert!(response.content.contains("completed successfully"));
        assert_eq!(response.model, "mock-model");
        // Should have tokens from both calls
        assert!(response.usage.input_tokens >= 25); // 10 + 15
        assert!(response.usage.output_tokens >= 45); // 20 + 25
    }

    #[test]
    fn loop_detection_recovery_message_logic() {
        assert!(is_parallelizable("read_file"));
        assert!(is_parallelizable("glob"));
        assert!(is_parallelizable("grep"));
        assert!(!is_parallelizable("bash"));
        assert!(!is_parallelizable("write_file"));
        assert!(!is_parallelizable("edit_file"));
    }

    /// The TUI streams (`chunk_tx` is set), so its tool calls reach the agent only
    /// through `drain_stream_to_response`. A provider-level test that inspects raw
    /// StreamEvents does NOT cover this: it can pass while the assembled
    /// `LLMResponse` carries no ToolUse block at all, which is exactly the
    /// "model says it will run ls, then nothing happens" bug.
    ///
    ///   cargo test --features ollama -- --ignored streamed_ollama_tool_call_survives_drain
    #[cfg(feature = "ollama")]
    #[tokio::test]
    #[ignore = "requires a running Ollama with a tool-capable model"]
    async fn streamed_ollama_tool_call_survives_drain() {
        use crate::llm::provider::{OllamaProvider, Provider, Tool};

        let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "ornith:9b".to_string());
        let provider = OllamaProvider::default_local().with_default_model(model.clone());

        let request = LLMRequest::new(
            &model,
            vec![Message::user("list all files into current folder")],
        )
        .with_tools(vec![Tool {
            name: "bash".to_string(),
            description: "Run a shell command".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {"command": {"type": "string"}},
                "required": ["command"],
            }),
        }])
        // Mirror the agent loop's request exactly: it sets max_tokens and a
        // system prompt, and the TUI streams. Any of these can change whether
        // the model emits a tool call at all.
        .with_max_tokens(4096)
        .with_system(crate::cli::SYSTEM_PROMPT.to_string())
        .with_streaming();

        let stream = provider.stream(request).await.expect("stream starts");
        let response = drain_stream_to_response(stream, None, &model)
            .await
            .expect("stream drains");

        let tool_uses: Vec<_> = response
            .content
            .iter()
            .filter(|b| matches!(b, ContentBlock::ToolUse { .. }))
            .collect();

        assert!(
            !tool_uses.is_empty(),
            "assembled response carried no ToolUse block, so the agent would log \
             'Found 0 tool uses' and end the turn. Blocks: {:?}, stop_reason: {:?}",
            response.content,
            response.stop_reason
        );
        assert_eq!(response.stop_reason, Some(StopReason::ToolUse));
    }

    #[tokio::test]
    async fn drain_stream_to_response_carries_perf_metrics_through() {
        use crate::llm::provider::types::MessageDelta;
        use crate::llm::provider::ContentDelta;

        let events: Vec<crate::llm::provider::Result<StreamEvent>> = vec![
            Ok(StreamEvent::ContentBlockStart {
                index: 0,
                content_block: ContentBlock::Text {
                    text: String::new(),
                },
            }),
            Ok(StreamEvent::ContentBlockDelta {
                index: 0,
                delta: ContentDelta::TextDelta {
                    text: "hi".to_string(),
                },
            }),
            Ok(StreamEvent::ContentBlockStop { index: 0 }),
            Ok(StreamEvent::MessageDelta {
                delta: MessageDelta {
                    stop_reason: Some(StopReason::EndTurn),
                    stop_sequence: None,
                },
                usage: TokenUsage {
                    input_tokens: 5,
                    output_tokens: 10,
                },
                perf_metrics: Some(PerfMetrics {
                    load_duration_ms: Some(50),
                    prompt_eval_duration_ms: Some(20),
                    eval_duration_ms: Some(200),
                    total_duration_ms: Some(270),
                    model_was_loaded: Some(true),
                }),
            }),
            Ok(StreamEvent::MessageStop),
        ];
        let stream: ProviderStream = Box::pin(futures::stream::iter(events));

        let response = drain_stream_to_response(stream, None, "mock-model")
            .await
            .unwrap();

        // This is the regression this test guards against: PerfMetrics
        // reported mid-stream by the provider must survive
        // drain_stream_to_response instead of being discarded (previously
        // hardcoded to `None` regardless of what the stream carried).
        let perf = response
            .perf_metrics
            .expect("perf metrics should survive draining the stream");
        assert_eq!(perf.eval_duration_ms, Some(200));
        assert_eq!(perf.model_was_loaded, Some(true));
    }

    #[tokio::test]
    async fn drain_stream_assembles_anthropic_tool_input_from_json_deltas() {
        use crate::llm::provider::types::MessageDelta;
        use crate::llm::provider::ContentDelta;

        // Anthropic streams a tool_use block with an empty `input`, then sends
        // the real arguments as `input_json_delta` fragments before the stop.
        let events: Vec<crate::llm::provider::Result<StreamEvent>> = vec![
            Ok(StreamEvent::ContentBlockStart {
                index: 0,
                content_block: ContentBlock::ToolUse {
                    id: "toolu_1".to_string(),
                    name: "read_file".to_string(),
                    input: serde_json::json!({}),
                },
            }),
            Ok(StreamEvent::ContentBlockDelta {
                index: 0,
                delta: ContentDelta::InputJsonDelta {
                    partial_json: "{\"file_path\":".to_string(),
                },
            }),
            Ok(StreamEvent::ContentBlockDelta {
                index: 0,
                delta: ContentDelta::InputJsonDelta {
                    partial_json: "\"src/main.rs\"}".to_string(),
                },
            }),
            Ok(StreamEvent::ContentBlockStop { index: 0 }),
            Ok(StreamEvent::MessageDelta {
                delta: MessageDelta {
                    stop_reason: Some(StopReason::ToolUse),
                    stop_sequence: None,
                },
                usage: TokenUsage {
                    input_tokens: 5,
                    output_tokens: 10,
                },
                perf_metrics: None,
            }),
            Ok(StreamEvent::MessageStop),
        ];
        let stream: ProviderStream = Box::pin(futures::stream::iter(events));

        let response = drain_stream_to_response(stream, None, "claude-3-5-sonnet")
            .await
            .unwrap();

        // Regression: the streamed tool call must carry its assembled arguments
        // instead of the empty `{}` from ContentBlockStart.
        let tool = response
            .content
            .iter()
            .find_map(|b| match b {
                ContentBlock::ToolUse { name, input, .. } => Some((name.clone(), input.clone())),
                _ => None,
            })
            .expect("response must contain a tool use block");
        assert_eq!(tool.0, "read_file");
        assert_eq!(
            tool.1,
            serde_json::json!({"file_path": "src/main.rs"}),
            "input_json_delta fragments must be assembled into the tool input"
        );
    }

    // ── Context budget, tool profile and overflow recovery ──────────────────

    /// Provider that answers from a script, one entry per call, and records
    /// every request it receives.
    struct ScriptedProvider {
        name: &'static str,
        model: &'static str,
        window: u32,
        script:
            std::sync::Mutex<std::collections::VecDeque<crate::llm::provider::Result<LLMResponse>>>,
        requests: std::sync::Mutex<Vec<LLMRequest>>,
    }

    impl ScriptedProvider {
        fn new(window: u32, script: Vec<crate::llm::provider::Result<LLMResponse>>) -> Self {
            Self {
                name: "scripted",
                model: "scripted-model",
                window,
                script: std::sync::Mutex::new(script.into()),
                requests: std::sync::Mutex::new(Vec::new()),
            }
        }

        fn named(mut self, name: &'static str, model: &'static str) -> Self {
            self.name = name;
            self.model = model;
            self
        }

        fn requests(&self) -> Vec<LLMRequest> {
            self.requests.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl Provider for ScriptedProvider {
        async fn complete(&self, request: LLMRequest) -> crate::llm::provider::Result<LLMResponse> {
            self.requests.lock().unwrap().push(request);
            self.script
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_else(|| Ok(text_reply("done")))
        }

        async fn stream(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<ProviderStream> {
            unimplemented!("tests use the non-streaming path")
        }

        fn name(&self) -> &str {
            self.name
        }

        fn default_model(&self) -> &str {
            self.model
        }

        fn supported_models(&self) -> Vec<String> {
            vec![self.model.to_string()]
        }

        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(self.window)
        }

        fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
            0.0
        }
    }

    fn text_reply(text: &str) -> LLMResponse {
        LLMResponse {
            id: "r".to_string(),
            model: "scripted-model".to_string(),
            content: vec![ContentBlock::Text {
                text: text.to_string(),
            }],
            stop_reason: Some(StopReason::EndTurn),
            usage: TokenUsage {
                input_tokens: 0,
                output_tokens: 1,
            },
            cache_metrics: None,
            perf_metrics: None,
        }
    }

    fn tool_call(name: &str, input: serde_json::Value) -> LLMResponse {
        LLMResponse {
            id: "r".to_string(),
            model: "scripted-model".to_string(),
            content: vec![ContentBlock::ToolUse {
                id: "call-1".to_string(),
                name: name.to_string(),
                input,
            }],
            stop_reason: Some(StopReason::ToolUse),
            usage: TokenUsage {
                input_tokens: 0,
                output_tokens: 1,
            },
            cache_metrics: None,
            perf_metrics: None,
        }
    }

    /// Tool with a configurable name and approval requirement.
    struct NamedTool {
        name: &'static str,
        requires_approval: bool,
    }

    #[async_trait]
    impl crate::llm::tools::Tool for NamedTool {
        fn name(&self) -> &str {
            self.name
        }

        fn description(&self) -> &str {
            "A named test tool"
        }

        fn input_schema(&self) -> serde_json::Value {
            serde_json::json!({"type": "object", "properties": {}})
        }

        fn capabilities(&self) -> Vec<ToolCapability> {
            vec![]
        }

        fn requires_approval(&self) -> bool {
            self.requires_approval
        }

        async fn execute(
            &self,
            _input: serde_json::Value,
            _context: &ToolExecutionContext,
        ) -> crate::llm::tools::Result<crate::llm::tools::ToolResult> {
            Ok(crate::llm::tools::ToolResult::success(format!(
                "{} ran",
                self.name
            )))
        }
    }

    fn registry_of(tools: &[(&'static str, bool)]) -> Arc<ToolRegistry> {
        let mut registry = ToolRegistry::new();
        for &(name, requires_approval) in tools {
            registry.register(Arc::new(NamedTool {
                name,
                requires_approval,
            }));
        }
        Arc::new(registry)
    }

    async fn session_for(context: &ServiceContext) -> Uuid {
        SessionService::new(context.clone())
            .create_session(Some("Test".to_string()))
            .await
            .unwrap()
            .id
    }

    async fn test_context() -> ServiceContext {
        let db = Database::connect_in_memory().await.unwrap();
        db.run_migrations().await.unwrap();
        ServiceContext::new(db.pool().clone())
    }

    fn tool_names(request: &LLMRequest) -> Vec<String> {
        let mut names: Vec<String> = request
            .tools
            .iter()
            .flatten()
            .map(|t| t.name.clone())
            .collect();
        names.sort();
        names
    }

    /// With the auto profile on a small window, requests carry only the core
    /// tools, and a call to a tool outside them gets an error result instead
    /// of running.
    #[tokio::test]
    async fn auto_profile_sends_core_tools_on_a_small_window() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            8192,
            vec![
                Ok(tool_call("web_fetch", serde_json::json!({}))),
                Ok(text_reply("answer")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(registry_of(&[
                ("read_file", false),
                ("grep", false),
                ("web_fetch", false),
            ]))
            .with_tool_profile(ToolProfile::Auto, Vec::new())
            .with_system_prompt("full prompt".to_string())
            .with_core_system_prompt("core prompt".to_string());

        let response = service
            .send_message_with_tools(session, "fetch it".to_string(), None)
            .await
            .unwrap();

        let requests = provider.requests();
        assert_eq!(tool_names(&requests[0]), vec!["grep", "read_file"]);
        assert!(requests[0]
            .system
            .as_deref()
            .unwrap()
            .starts_with("core prompt"));

        // The hidden tool did not run: its result is an error naming the
        // tools that are available.
        let result = requests[1]
            .messages
            .iter()
            .flat_map(|m| m.content.iter())
            .find_map(|b| match b {
                ContentBlock::ToolResult {
                    content, is_error, ..
                } => Some((content, *is_error)),
                _ => None,
            })
            .unwrap();
        assert_eq!(result.1, Some(true));
        assert!(result.0.contains("not available"), "got: {}", result.0);
        assert!(!result.0.contains("web_fetch ran"));

        let usage = response.context.expect("context usage is reported");
        assert!(usage.core_tools_only);
        assert_eq!(usage.tool_count, 2);
        assert_eq!(usage.window, 8192);
    }

    #[tokio::test]
    async fn auto_profile_sends_every_tool_on_a_large_window() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(128_000, vec![Ok(text_reply("hi"))]));
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(registry_of(&[("read_file", false), ("web_fetch", false)]))
            .with_tool_profile(ToolProfile::Auto, Vec::new());

        service
            .send_message_with_tools(session, "hi".to_string(), None)
            .await
            .unwrap();

        assert_eq!(
            tool_names(&provider.requests()[0]),
            vec!["read_file", "web_fetch"]
        );
        assert_eq!(service.exposed_tool_names(), vec!["read_file", "web_fetch"]);
    }

    /// A model that calls a tool by an alias (`Bash`) must reach the approval
    /// gate under the registered name: AutoPlan's high-risk check matches
    /// registered names only, so the alias used to skip it.
    #[tokio::test]
    async fn approval_callback_sees_the_canonical_tool_name() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call("Bash", serde_json::json!({}))),
                Ok(text_reply("done")),
            ],
        ));
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let seen_in_callback = seen.clone();
        let callback: ApprovalCallback = Arc::new(move |info: ToolApprovalInfo| {
            seen_in_callback.lock().unwrap().push(info.tool_name);
            Box::pin(async { Ok(true) })
        });
        let service = AgentService::new(provider, context)
            .with_tool_registry(registry_of(&[("bash", true)]))
            .with_approval_callback(Some(callback));

        service
            .send_message_with_tools(session, "run it".to_string(), None)
            .await
            .unwrap();

        assert_eq!(*seen.lock().unwrap(), vec!["bash".to_string()]);
    }

    /// When the provider says the prompt overflowed its window, the loop
    /// compacts the history and retries the same round instead of failing.
    #[tokio::test]
    async fn context_overflow_compacts_and_retries_the_round() {
        use crate::llm::provider::ProviderError;

        let context = test_context().await;
        let session = session_for(&context).await;
        let messages = MessageService::new(context.clone());
        for i in 0..6 {
            messages
                .create_message(session, "user".to_string(), format!("question {i}"))
                .await
                .unwrap();
            messages
                .create_message(session, "assistant".to_string(), format!("answer {i}"))
                .await
                .unwrap();
        }

        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Err(ProviderError::ContextLengthExceeded(200_000)),
                Ok(text_reply("fits now")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context.clone());

        let response = service
            .send_message_with_tools(session, "next question".to_string(), None)
            .await
            .unwrap();

        assert_eq!(response.content, "fits now");
        let requests = provider.requests();
        assert_eq!(requests.len(), 2, "one failed attempt, one retry");
        assert!(
            requests[1].messages.len() < requests[0].messages.len(),
            "the retry must carry less history"
        );
        // The retried request opens with a user message and carries the
        // summary of what was dropped in its system prompt.
        assert!(matches!(
            requests[1].messages[0].role,
            crate::llm::provider::Role::User
        ));
        assert!(requests[1]
            .system
            .as_deref()
            .unwrap()
            .contains("Earlier in this conversation (compacted)"));

        let records: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM compaction_records")
            .fetch_one(&*context.pool)
            .await
            .unwrap();
        assert_eq!(records, 1);
    }

    /// An overflow that compaction cannot fix (nothing left to drop) is
    /// reported, not retried forever.
    #[tokio::test]
    async fn context_overflow_with_nothing_to_free_is_an_error() {
        use crate::llm::provider::ProviderError;

        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            8192,
            vec![
                Err(ProviderError::ContextLengthExceeded(9000)),
                Ok(text_reply("unreachable")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context);

        let result = service
            .send_message_with_tools(session, "hello".to_string(), None)
            .await;

        assert!(matches!(result, Err(AgentError::Provider(_))));
        assert_eq!(provider.requests().len(), 1);
    }

    /// Before each round, a tight budget elides old tool output first.
    #[tokio::test]
    async fn make_room_elides_old_tool_output_when_the_budget_is_tight() {
        let context = test_context().await;
        let service = AgentService::new(Arc::new(MockProvider), context);
        let mut agent_context = AgentContext::new(Uuid::new_v4(), 8192);
        agent_context.add_message(Message::user("read everything"));
        let big = "fn f() {}\n".repeat(300);
        for i in 0..6 {
            agent_context.add_message(Message {
                role: crate::llm::provider::Role::Assistant,
                content: vec![ContentBlock::ToolUse {
                    id: format!("c{i}"),
                    name: "read_file".to_string(),
                    input: serde_json::json!({}),
                }],
            });
            agent_context.add_message(Message {
                role: crate::llm::provider::Role::User,
                content: vec![ContentBlock::ToolResult {
                    tool_use_id: format!("c{i}"),
                    content: big.clone(),
                    is_error: None,
                }],
            });
        }
        let tools = RequestTools::new(Vec::new());
        assert!(service.measure_budget(&agent_context, &tools).is_tight());
        let before = agent_context.token_count;

        assert!(service.make_room(&mut agent_context, &tools, 0).await);

        assert!(agent_context.token_count < before);
        assert!(!service.measure_budget(&agent_context, &tools).is_tight());
    }

    /// The provider's own prompt count corrects later estimates upward, and a
    /// provider switch starts over.
    #[tokio::test]
    async fn calibration_learns_from_the_provider_and_resets_on_switch() {
        let context = test_context().await;
        let mut service = AgentService::new(Arc::new(MockProvider), context);
        let budget = ContextBudget::from_counts(4096, 500, 500, 2, 0, 1.0);
        let mut response = text_reply("x");
        response.usage.input_tokens = 1500;

        service.record_calibration(&response, &budget);
        assert!((service.calibration() - 1.5).abs() < 1e-9);

        // A response served partly from the provider's prompt cache says
        // nothing reliable about the prompt size.
        response.usage.input_tokens = 1900;
        response.cache_metrics = Some(crate::llm::provider::types::CacheMetrics {
            read_tokens: 1000,
            creation_tokens: 0,
        });
        service.record_calibration(&response, &budget);
        assert!((service.calibration() - 1.5).abs() < 1e-9);

        service.set_provider(Arc::new(MockProvider));
        assert_eq!(service.calibration(), 1.0);
    }

    // ── Middleware, cancellation, tool log, project memory (P1) ──────────────

    fn last_user_text(request: &LLMRequest) -> String {
        request
            .messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, crate::llm::provider::Role::User))
            .and_then(|m| {
                m.content.iter().find_map(|b| match b {
                    ContentBlock::Text { text } => Some(text.clone()),
                    _ => None,
                })
            })
            .unwrap_or_default()
    }

    /// At the round limit the loop no longer fails with
    /// `MaxIterationsExceeded`: the pending tool calls get "not run" results
    /// and the model answers, without tools, from the work done so far.
    #[tokio::test]
    async fn round_limit_ends_with_an_answer_instead_of_an_error() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call("read_file", serde_json::json!({"path": "a.rs"}))),
                Ok(text_reply("Here is what I found so far.")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(registry_of(&[("read_file", false)]))
            .with_max_tool_iterations(1);

        let response = service
            .send_message_with_tools(session, "read the files".to_string(), None)
            .await
            .unwrap();

        assert!(response.content.starts_with("Here is what I found so far."));
        assert!(
            response
                .content
                .contains("Stopped at the limit of 1 tool rounds"),
            "the user is told why: {}",
            response.content
        );
        let requests = provider.requests();
        assert_eq!(requests.len(), 2, "one tool round, then the final answer");
        assert!(
            requests[1].tools.is_none(),
            "the final call offers no tools"
        );
        assert!(last_user_text(&requests[1]).contains("reached the limit"));
    }

    /// When the loop guard stops a stuck model, every pending tool use still
    /// gets a result, so the final request is valid for strict providers.
    #[tokio::test]
    async fn stopping_mid_round_answers_every_pending_tool_call() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let call = || Ok(tool_call("write_file", serde_json::json!({"path": "a.rs"})));
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![call(), call(), Ok(text_reply("Stopping."))],
        ));
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(WritingTool));
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(Arc::new(registry))
            .with_auto_approve_tools(true);

        let response = service
            .send_message_with_tools(session, "write it".to_string(), None)
            .await
            .unwrap();

        assert_eq!(response.content, "Stopping.");
        let final_request = provider.requests().pop().unwrap();
        let uses: Vec<&String> = final_request
            .messages
            .iter()
            .flat_map(|m| m.content.iter())
            .filter_map(|b| match b {
                ContentBlock::ToolUse { id, .. } => Some(id),
                _ => None,
            })
            .collect();
        let results: Vec<&String> = final_request
            .messages
            .iter()
            .flat_map(|m| m.content.iter())
            .filter_map(|b| match b {
                ContentBlock::ToolResult { tool_use_id, .. } => Some(tool_use_id),
                _ => None,
            })
            .collect();
        assert_eq!(uses.len(), results.len(), "every tool use has its result");
        assert!(last_user_text(&final_request).contains("Loop detected"));
    }

    /// Tool that writes files (for the loop guard's modification threshold).
    struct WritingTool;

    #[async_trait]
    impl crate::llm::tools::Tool for WritingTool {
        fn name(&self) -> &str {
            "write_file"
        }
        fn description(&self) -> &str {
            "Writes a file"
        }
        fn input_schema(&self) -> serde_json::Value {
            serde_json::json!({"type": "object", "properties": {"path": {"type": "string"}}})
        }
        fn capabilities(&self) -> Vec<ToolCapability> {
            vec![ToolCapability::WriteFiles]
        }
        fn requires_approval(&self) -> bool {
            false
        }
        async fn execute(
            &self,
            _input: serde_json::Value,
            _context: &ToolExecutionContext,
        ) -> crate::llm::tools::Result<crate::llm::tools::ToolResult> {
            Ok(crate::llm::tools::ToolResult::success(
                "written".to_string(),
            ))
        }
    }

    #[tokio::test]
    async fn a_cancelled_request_stops_with_cancelled() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(128_000, vec![Ok(text_reply("hi"))]));
        let service = AgentService::new(provider.clone(), context);
        let (tx, _rx) = mpsc::unbounded_channel();
        let cancel = CancellationToken::new();
        cancel.cancel();

        let result = service
            .send_message_with_tools_and_mode_streaming(
                session,
                "hello".to_string(),
                None,
                false,
                tx,
                cancel,
            )
            .await;

        assert!(matches!(result, Err(AgentError::Cancelled)));
        assert!(provider.requests().is_empty(), "nothing was sent");
    }

    /// Cancelling while an approval prompt is open ends the request; it does
    /// not wait for an answer that will never come.
    #[tokio::test]
    async fn cancelling_during_an_approval_prompt_ends_the_request() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![Ok(tool_call("bash", serde_json::json!({"command": "ls"})))],
        ));
        // The user cancels while the prompt is open, and never answers it.
        // Cancelling from inside the callback, rather than on a timer, makes
        // sure the request really reached the prompt: on a timer it could
        // be cancelled before the first model call and pass without testing
        // anything.
        let cancel = CancellationToken::new();
        let canceller = cancel.clone();
        let callback: ApprovalCallback = Arc::new(move |_info: ToolApprovalInfo| {
            canceller.cancel();
            Box::pin(std::future::pending())
        });
        let service = AgentService::new(provider.clone(), context.clone())
            .with_tool_registry(registry_of(&[("bash", true)]))
            .with_approval_callback(Some(callback));

        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            service.send_message_with_tools_cancellable(
                session,
                "list".to_string(),
                None,
                false,
                cancel,
            ),
        )
        .await
        .expect("cancellation must not hang");

        assert!(matches!(result, Err(AgentError::Cancelled)));
        assert_eq!(provider.requests.lock().unwrap().len(), 1);
    }

    /// With verification on, a reply after a file edit is held back until a
    /// check command has passed.
    #[tokio::test]
    async fn verification_sends_the_model_back_to_check_its_edit() {
        let workdir = tempfile::tempdir().unwrap();
        std::fs::write(workdir.path().join("Cargo.toml"), "[package]").unwrap();
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call(
                    "edit_file",
                    serde_json::json!({"path": "src/lib.rs"}),
                )),
                Ok(text_reply("Done, I fixed it.")),
                Ok(tool_call(
                    "bash",
                    serde_json::json!({"command": "cargo check"}),
                )),
                Ok(text_reply("Fixed, and cargo check passes.")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(registry_of(&[("edit_file", false), ("bash", false)]))
            .with_working_directory(workdir.path().to_path_buf())
            .with_verification(true, None);

        let response = service
            .send_message_with_tools(session, "fix the bug".to_string(), None)
            .await
            .unwrap();

        assert_eq!(response.content, "Fixed, and cargo check passes.");
        let requests = provider.requests();
        assert_eq!(requests.len(), 4);
        let nudge = last_user_text(&requests[2]);
        assert!(nudge.contains("cargo check"), "{nudge}");
        assert!(nudge.contains("src/lib.rs"), "{nudge}");
    }

    /// Without verification, the first reply after an edit is the answer.
    #[tokio::test]
    async fn verification_is_off_unless_enabled() {
        let workdir = tempfile::tempdir().unwrap();
        std::fs::write(workdir.path().join("Cargo.toml"), "[package]").unwrap();
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call("edit_file", serde_json::json!({"path": "a.rs"}))),
                Ok(text_reply("Done.")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(registry_of(&[("edit_file", false), ("bash", false)]))
            .with_working_directory(workdir.path().to_path_buf());

        let response = service
            .send_message_with_tools(session, "fix".to_string(), None)
            .await
            .unwrap();
        assert_eq!(response.content, "Done.");
        assert_eq!(provider.requests().len(), 2);
    }

    /// The tool calls behind an answer are stored with it and shown to the
    /// model on the next request; requests that edit files are logged to the
    /// progress file.
    #[tokio::test]
    async fn tool_work_is_remembered_across_requests() {
        let workdir = tempfile::tempdir().unwrap();
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call(
                    "edit_file",
                    serde_json::json!({"path": "src/main.rs"}),
                )),
                Ok(text_reply("Renamed the function.")),
                Ok(text_reply("It was in src/main.rs.")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context.clone())
            .with_tool_registry(registry_of(&[("edit_file", false)]))
            .with_working_directory(workdir.path().to_path_buf())
            .with_progress_file(true);

        service
            .send_message_with_tools(session, "rename main_loop".to_string(), None)
            .await
            .unwrap();
        service
            .send_message_with_tools(session, "where was it?".to_string(), None)
            .await
            .unwrap();

        let stored = MessageService::new(context)
            .list_messages_for_session(session)
            .await
            .unwrap();
        let answer = stored.iter().find(|m| m.role == "assistant").unwrap();
        assert_eq!(answer.tool_log.as_deref(), Some("edit_file src/main.rs"));

        let second = provider.requests().pop().unwrap();
        let replayed = second
            .messages
            .iter()
            .filter_map(|m| match &m.content[0] {
                ContentBlock::Text { text }
                    if matches!(m.role, crate::llm::provider::Role::Assistant) =>
                {
                    Some(text.clone())
                }
                _ => None,
            })
            .next()
            .unwrap();
        assert!(
            replayed.starts_with("(Tools used before this answer: edit_file src/main.rs)"),
            "{replayed}"
        );

        let log = std::fs::read_to_string(
            workdir
                .path()
                .join(super::super::project_memory::PROGRESS_FILE),
        )
        .unwrap();
        assert!(log.contains("rename main_loop"));
        assert!(log.contains("- Tools: edit_file src/main.rs"));
        assert!(log.contains("- Result: Renamed the function."));
    }

    #[tokio::test]
    async fn project_notes_are_added_to_the_system_prompt_when_enabled() {
        let workdir = tempfile::tempdir().unwrap();
        std::fs::write(
            workdir.path().join("AGENTS.md"),
            "Run cargo fmt before committing.",
        )
        .unwrap();
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            8192,
            vec![Ok(text_reply("ok")), Ok(text_reply("ok"))],
        ));
        let service = AgentService::new(provider.clone(), context)
            .with_system_prompt("base prompt".to_string())
            .with_working_directory(workdir.path().to_path_buf())
            .with_project_notes(true);

        service
            .send_message_with_tools(session, "hi".to_string(), None)
            .await
            .unwrap();
        let system = provider.requests()[0].system.clone().unwrap();
        assert!(system.contains("## Project notes"), "{system}");
        assert!(system.contains("Run cargo fmt before committing."));

        let service = service.with_project_notes(false);
        service
            .send_message_with_tools(session, "hi".to_string(), None)
            .await
            .unwrap();
        let system = provider.requests()[1].system.clone().unwrap();
        assert!(!system.contains("## Project notes"));
    }

    /// Tool with a typed schema that rejects input missing its `path`.
    struct StrictTool;

    #[async_trait]
    impl crate::llm::tools::Tool for StrictTool {
        fn name(&self) -> &str {
            "read_file"
        }
        fn description(&self) -> &str {
            "Reads a file"
        }
        fn input_schema(&self) -> serde_json::Value {
            serde_json::json!({
                "type": "object",
                "properties": {"path": {"type": "string"}, "limit": {"type": "integer"}},
                "required": ["path"]
            })
        }
        fn capabilities(&self) -> Vec<ToolCapability> {
            vec![ToolCapability::ReadFiles]
        }
        fn requires_approval(&self) -> bool {
            false
        }
        async fn execute(
            &self,
            input: serde_json::Value,
            _context: &ToolExecutionContext,
        ) -> crate::llm::tools::Result<crate::llm::tools::ToolResult> {
            match input.get("path").and_then(|p| p.as_str()) {
                Some(path) => Ok(crate::llm::tools::ToolResult::success(format!(
                    "read {path}"
                ))),
                None => Err(crate::llm::tools::ToolError::InvalidInput(
                    "missing field `path`".to_string(),
                )),
            }
        }
    }

    fn tool_results(request: &LLMRequest) -> Vec<String> {
        request
            .messages
            .iter()
            .flat_map(|m| m.content.iter())
            .filter_map(|b| match b {
                ContentBlock::ToolResult { content, .. } => Some(content.clone()),
                _ => None,
            })
            .collect()
    }

    /// Malformed arguments get an error that lists the expected arguments,
    /// whether the check catches them first or the tool rejects them.
    #[tokio::test]
    async fn bad_tool_arguments_get_the_expected_arguments_back() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call(
                    "read_file",
                    serde_json::json!({"path": ["a.rs"]}),
                )),
                Ok(tool_call("read_file", serde_json::json!({"limit": 5}))),
                Ok(text_reply("done")),
            ],
        ));
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(StrictTool));
        let service =
            AgentService::new(provider.clone(), context).with_tool_registry(Arc::new(registry));

        service
            .send_message_with_tools(session, "read".to_string(), None)
            .await
            .unwrap();

        let results = tool_results(&provider.requests()[2]);
        assert_eq!(results.len(), 2);
        assert!(
            results[0].contains("field 'path' must be string, got a list"),
            "{}",
            results[0]
        );
        assert!(results[0].contains("path (string, required)"));
        assert!(
            results[1].contains("missing field `path`"),
            "{}",
            results[1]
        );
        assert!(results[1].contains("Expected arguments for read_file: path (string, required)"));
    }

    // ── Sub-agents, profiles, router, fallback (P2) ──────────────────────────

    /// A registry with the `agent` tool plus named test tools.
    fn agent_registry(tools: &[(&'static str, bool)]) -> Arc<ToolRegistry> {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(crate::llm::tools::agent::AgentTool));
        for &(name, requires_approval) in tools {
            registry.register(Arc::new(NamedTool {
                name,
                requires_approval,
            }));
        }
        Arc::new(registry)
    }

    /// The sub-agent's answer comes back to the parent as the tool result;
    /// the explorer profile only sees its read-only tools; the task and the
    /// answer are kept under .crustly/agents.
    #[tokio::test]
    async fn a_sub_agent_s_answer_comes_back_to_the_parent() {
        let workdir = tempfile::tempdir().unwrap();
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call(
                    "agent",
                    serde_json::json!({
                        "description": "find usages",
                        "prompt": "Where is parse_config used?",
                        "profile": "explorer"
                    }),
                )),
                Ok(text_reply("parse_config is called in src/main.rs:12")),
                Ok(text_reply("It is called from src/main.rs.")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(agent_registry(&[
                ("read_file", false),
                ("grep", false),
                ("edit_file", false),
            ]))
            .with_working_directory(workdir.path().to_path_buf())
            .with_system_prompt("parent prompt".to_string());

        let response = service
            .send_message_with_tools(session, "where is parse_config used?".to_string(), None)
            .await
            .unwrap();

        assert_eq!(response.content, "It is called from src/main.rs.");
        let requests = provider.requests();
        assert_eq!(requests.len(), 3, "parent, sub-agent, parent");
        assert_eq!(tool_names(&requests[1]), vec!["grep", "read_file"]);
        let child_system = requests[1].system.clone().unwrap();
        assert!(child_system.contains("## Your role"), "{child_system}");
        assert!(child_system.contains("explorer sub-agent"));
        let results = tool_results(&requests[2]);
        assert!(
            results[0].contains("parse_config is called in src/main.rs:12"),
            "{}",
            results[0]
        );

        let dir = workdir.path().join(".crustly/agents");
        let files: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        assert_eq!(files.len(), 2, "{files:?}");
        let md = files.iter().find(|f| f.ends_with(".md")).unwrap();
        let task = std::fs::read_to_string(dir.join(md)).unwrap();
        assert!(task.contains("Where is parse_config used?"));
        assert!(task.contains("## Answer\n\nparse_config is called in src/main.rs:12"));
    }

    /// Without an approval prompt (crustly run without --auto-approve), a
    /// sub-agent's approval-requiring tools are refused, never run.
    #[tokio::test]
    async fn sub_agents_without_an_approval_prompt_cannot_run_gated_tools() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call(
                    "agent",
                    serde_json::json!({
                        "description": "run tests",
                        "prompt": "Run the tests",
                        "profile": "implementer"
                    }),
                )),
                Ok(tool_call(
                    "bash",
                    serde_json::json!({"command": "cargo test"}),
                )),
                Ok(text_reply("I could not run the tests.")),
                Ok(text_reply("The sub-agent could not run them.")),
            ],
        ));
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(agent_registry(&[("bash", true)]));

        service
            .send_message_with_tools(session, "test it".to_string(), None)
            .await
            .unwrap();

        let child_results = tool_results(&provider.requests()[2]);
        assert!(
            child_results[0].contains("requires approval"),
            "{}",
            child_results[0]
        );
    }

    /// With an approval prompt, a sub-agent's gated tool calls go through it,
    /// labelled with the sub-agent's task.
    #[tokio::test]
    async fn sub_agent_tool_calls_are_approved_through_the_parent_s_prompt() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![
                Ok(tool_call(
                    "agent",
                    serde_json::json!({
                        "description": "run tests",
                        "prompt": "Run the tests",
                        "profile": "implementer"
                    }),
                )),
                Ok(tool_call(
                    "bash",
                    serde_json::json!({"command": "cargo test"}),
                )),
                Ok(text_reply("All tests pass.")),
                Ok(text_reply("Tests pass.")),
            ],
        ));
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let seen_in_callback = seen.clone();
        let callback: ApprovalCallback = Arc::new(move |info: ToolApprovalInfo| {
            seen_in_callback
                .lock()
                .unwrap()
                .push(info.tool_description.clone());
            Box::pin(async { Ok(true) })
        });
        let service = AgentService::new(provider.clone(), context)
            .with_tool_registry(agent_registry(&[("bash", true)]))
            .with_approval_callback(Some(callback));

        service
            .send_message_with_tools(session, "test it".to_string(), None)
            .await
            .unwrap();

        let seen = seen.lock().unwrap();
        assert_eq!(seen.len(), 1);
        assert!(seen[0].starts_with("[sub-agent: run tests]"), "{}", seen[0]);
        assert!(tool_results(&provider.requests()[2])[0].contains("bash ran"));
    }

    fn task(description: &str, profile: Option<&str>) -> crate::llm::tools::SubAgentTask {
        crate::llm::tools::SubAgentTask {
            id: Uuid::new_v4(),
            description: description.to_string(),
            prompt: format!("Please {description}"),
            profile: profile.map(str::to_string),
            read_only: false,
        }
    }

    fn launcher_for(service: &AgentService) -> subagents::AgentServiceLauncher {
        subagents::AgentServiceLauncher::new(service.subagents.clone(), service.child_template())
    }

    #[tokio::test]
    async fn a_background_sub_agent_is_collected_with_its_report() {
        use crate::llm::tools::{SubAgentLauncher, SubAgentStatus};

        let workdir = tempfile::tempdir().unwrap();
        let context = test_context().await;
        let provider = Arc::new(ScriptedProvider::new(
            128_000,
            vec![Ok(text_reply("Done in the background."))],
        ));
        let service = AgentService::new(provider, context)
            .with_working_directory(workdir.path().to_path_buf());
        let launcher = launcher_for(&service);
        let task = task("summarize the readme", None);
        let id = task.id;

        launcher
            .start(task, CancellationToken::new())
            .await
            .unwrap();
        let report = launcher.report(id, true).await.unwrap();

        assert_eq!(report.status, SubAgentStatus::Completed);
        assert_eq!(report.answer.as_deref(), Some("Done in the background."));
        assert_eq!(report.profile, "general");
        assert_eq!(launcher.list().len(), 1);
        assert!(!launcher.cancel(id), "a finished sub-agent is not running");
    }

    #[tokio::test]
    async fn an_unknown_profile_fails_with_the_available_ones() {
        use crate::llm::tools::{SubAgentLauncher, SubAgentStatus};

        let context = test_context().await;
        let provider = Arc::new(ScriptedProvider::new(128_000, vec![]));
        let service = AgentService::new(provider.clone(), context)
            .with_working_directory(tempfile::tempdir().unwrap().path().to_path_buf());

        let report = launcher_for(&service)
            .run(task("x", Some("wizard")), CancellationToken::new())
            .await;

        assert_eq!(report.status, SubAgentStatus::Failed);
        let error = report.error.unwrap();
        assert!(error.contains("unknown profile 'wizard'"), "{error}");
        assert!(error.contains("explorer"));
        assert!(provider.requests().is_empty());
    }

    /// Provider whose calls wait until the test lets them through.
    struct GatedProvider {
        gate: Arc<tokio::sync::Semaphore>,
    }

    #[async_trait]
    impl Provider for GatedProvider {
        async fn complete(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<LLMResponse> {
            self.gate.acquire().await.unwrap().forget();
            Ok(text_reply("released"))
        }
        async fn stream(
            &self,
            _request: LLMRequest,
        ) -> crate::llm::provider::Result<ProviderStream> {
            unimplemented!()
        }
        fn name(&self) -> &str {
            "gated"
        }
        fn default_model(&self) -> &str {
            "gated-model"
        }
        fn supported_models(&self) -> Vec<String> {
            vec!["gated-model".to_string()]
        }
        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(128_000)
        }
        fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
            0.0
        }
    }

    async fn wait_for_status(
        launcher: &subagents::AgentServiceLauncher,
        id: Uuid,
        status: crate::llm::tools::SubAgentStatus,
    ) {
        use crate::llm::tools::SubAgentLauncher;
        for _ in 0..200 {
            if launcher.report(id, false).await.map(|r| r.status) == Some(status) {
                return;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        panic!("sub-agent {id} never reached {status:?}");
    }

    /// With one slot (the default), a second sub-agent waits until the
    /// first is done: one local GPU serializes inference anyway.
    #[tokio::test]
    async fn sub_agents_run_one_at_a_time_by_default() {
        use crate::llm::tools::{SubAgentLauncher, SubAgentStatus};

        let workdir = tempfile::tempdir().unwrap();
        let context = test_context().await;
        let gate = Arc::new(tokio::sync::Semaphore::new(0));
        let service = AgentService::new(Arc::new(GatedProvider { gate: gate.clone() }), context)
            .with_working_directory(workdir.path().to_path_buf());
        let launcher = launcher_for(&service);
        let first = task("first", None);
        let second = task("second", None);
        let (first_id, second_id) = (first.id, second.id);

        launcher
            .start(first, CancellationToken::new())
            .await
            .unwrap();
        wait_for_status(&launcher, first_id, SubAgentStatus::Running).await;
        launcher
            .start(second, CancellationToken::new())
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        assert_eq!(
            launcher.report(second_id, false).await.unwrap().status,
            SubAgentStatus::Queued
        );

        gate.add_permits(2);
        assert_eq!(
            launcher.report(first_id, true).await.unwrap().status,
            SubAgentStatus::Completed
        );
        assert_eq!(
            launcher.report(second_id, true).await.unwrap().status,
            SubAgentStatus::Completed
        );
    }

    #[tokio::test]
    async fn a_running_sub_agent_can_be_cancelled() {
        use crate::llm::tools::{SubAgentLauncher, SubAgentStatus};

        let workdir = tempfile::tempdir().unwrap();
        let context = test_context().await;
        let gate = Arc::new(tokio::sync::Semaphore::new(0));
        let service = AgentService::new(Arc::new(GatedProvider { gate }), context)
            .with_working_directory(workdir.path().to_path_buf());
        let launcher = launcher_for(&service);
        let task = task("never finishes", None);
        let id = task.id;

        launcher
            .start(task, CancellationToken::new())
            .await
            .unwrap();
        wait_for_status(&launcher, id, SubAgentStatus::Running).await;
        assert!(launcher.cancel(id));

        let report =
            tokio::time::timeout(std::time::Duration::from_secs(5), launcher.report(id, true))
                .await
                .expect("cancellation must not hang")
                .unwrap();
        assert_eq!(report.status, SubAgentStatus::Cancelled);
    }

    fn fallback_to(provider: Arc<ScriptedProvider>, on_overflow: bool) -> Fallback {
        Fallback {
            provider,
            on_error: true,
            on_overflow,
        }
    }

    /// When the main provider is unreachable or failing, the round goes to
    /// the fallback, with the fallback's own model, and the user is told.
    #[tokio::test]
    async fn a_failing_provider_falls_back_and_says_so() {
        use crate::llm::provider::ProviderError;

        let context = test_context().await;
        let session = session_for(&context).await;
        let local = Arc::new(
            ScriptedProvider::new(8192, vec![Err(ProviderError::Timeout(120))])
                .named("ollama", "qwen2.5-coder:7b"),
        );
        let cloud = Arc::new(
            ScriptedProvider::new(200_000, vec![Ok(text_reply("Answer from the cloud."))])
                .named("anthropic", "claude-sonnet"),
        );
        let service = AgentService::new(local.clone(), context)
            .with_fallback(fallback_to(cloud.clone(), true));

        let response = service
            .send_message_with_tools(session, "hello".to_string(), None)
            .await
            .unwrap();

        assert!(response.content.starts_with("Answer from the cloud."));
        assert!(
            response
                .content
                .contains("Answered by the fallback provider anthropic"),
            "{}",
            response.content
        );
        assert_eq!(response.provider_name, "anthropic");
        let sent = cloud.requests();
        assert_eq!(
            sent[0].model, "claude-sonnet",
            "never the local model's name"
        );
    }

    /// An overflow is first handled locally by compaction; only when
    /// nothing can be freed does the round go to the fallback, and only if
    /// `on_overflow` allows it.
    #[tokio::test]
    async fn an_overflow_falls_back_only_when_allowed() {
        use crate::llm::provider::ProviderError;

        for on_overflow in [true, false] {
            let context = test_context().await;
            let session = session_for(&context).await;
            let local = Arc::new(ScriptedProvider::new(
                8192,
                vec![Err(ProviderError::ContextLengthExceeded(9000))],
            ));
            let cloud = Arc::new(
                ScriptedProvider::new(200_000, vec![Ok(text_reply("Fits in the cloud."))])
                    .named("anthropic", "claude-sonnet"),
            );
            let service = AgentService::new(local, context)
                .with_fallback(fallback_to(cloud.clone(), on_overflow));

            let result = service
                .send_message_with_tools(session, "hello".to_string(), None)
                .await;

            if on_overflow {
                assert!(result.unwrap().content.starts_with("Fits in the cloud."));
            } else {
                assert!(matches!(result, Err(AgentError::Provider(_))));
                assert!(cloud.requests().is_empty());
            }
        }
    }

    #[tokio::test]
    async fn errors_that_a_retry_cannot_fix_do_not_fall_back() {
        use crate::llm::provider::ProviderError;

        let context = test_context().await;
        let session = session_for(&context).await;
        let local = Arc::new(ScriptedProvider::new(
            8192,
            vec![Err(ProviderError::InvalidApiKey)],
        ));
        let cloud = Arc::new(ScriptedProvider::new(200_000, vec![]));
        let service =
            AgentService::new(local, context).with_fallback(fallback_to(cloud.clone(), true));

        let result = service
            .send_message_with_tools(session, "hello".to_string(), None)
            .await;

        assert!(matches!(result, Err(AgentError::Provider(_))));
        assert!(cloud.requests().is_empty());
    }

    /// With the router on, the request's wording picks the tier's model; an
    /// explicit model (a sub-agent profile) wins.
    #[tokio::test]
    async fn the_router_picks_the_model_for_the_request() {
        let context = test_context().await;
        let session = session_for(&context).await;
        let provider = Arc::new(ScriptedProvider::new(128_000, vec![]));
        let service = AgentService::new(provider.clone(), context).with_model_router(
            ModelRouter::new("p", "small-model", "p", "mid-model", "p", "big-model"),
        );

        for prompt in [
            "what is a trait?",
            "refactor the parser module",
            "explain this",
        ] {
            service
                .send_message_with_tools(session, prompt.to_string(), None)
                .await
                .unwrap();
        }
        service
            .send_message_with_tools(
                session,
                "what is x?".to_string(),
                Some("pinned".to_string()),
            )
            .await
            .unwrap();

        let models: Vec<String> = provider.requests().into_iter().map(|r| r.model).collect();
        assert_eq!(
            models,
            vec!["small-model", "big-model", "mid-model", "pinned"]
        );
    }
}
