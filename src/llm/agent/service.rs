//! Agent Service Implementation
//!
//! Core service for managing AI agent conversations, coordinating between
//! LLM providers, context management, and data persistence.

use super::context::AgentContext;
use super::error::{AgentError, Result};
use crate::llm::provider::router::ModelRouter;
use crate::llm::provider::{
    ContentBlock, ContentDelta, LLMRequest, LLMResponse, Message, PerfMetrics, Provider,
    ProviderStream, StopReason, StreamEvent, TokenUsage,
};
use crate::llm::tools::cache::{CacheKey, ToolResultCache, ToolTtlConfig};
use crate::llm::tools::{ToolCapability, ToolExecutionContext, ToolRegistry};
use crate::services::{MessageService, ServiceContext, SessionService};
use futures::future::join_all;
use futures::StreamExt as _;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

/// True when the capabilities can change files or system state, meaning
/// cached read results may be stale after a tool with them runs.
fn has_mutating_capability(caps: &[ToolCapability]) -> bool {
    caps.iter().any(|c| {
        matches!(
            c,
            ToolCapability::WriteFiles
                | ToolCapability::ExecuteShell
                | ToolCapability::SystemModification
        )
    })
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

    /// SQLite pool for compaction writes (T032)
    pool: Option<Arc<sqlx::SqlitePool>>,

    /// Whether this service may wire a SubAgentLauncher into tool contexts.
    /// Always false for services created by AgentServiceLauncher to prevent
    /// recursive sub-agent spawning.
    allow_sub_agents: bool,
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
            pool: None,
            allow_sub_agents: true,
        }
    }

    /// Enable tier-based model routing (T046)
    pub fn with_model_router(mut self, router: ModelRouter) -> Self {
        self.model_router = Some(router);
        self
    }

    /// Enable compaction with a DB pool (T032)
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
        let base = self.default_system_prompt.as_ref()?;
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
        let (_model_name, request, message_service, session_service) = self
            .prepare_message_context(session_id, user_message, model)
            .await?;

        // Send to provider
        let response = self
            .provider
            .complete(request)
            .await
            .map_err(AgentError::Provider)?;

        // Extract text and optional thinking from response
        let assistant_text = Self::extract_text_from_response(&response);
        let thinking_text = Self::extract_thinking_from_response(&response);

        // Save assistant response to database
        let assistant_db_msg = message_service
            .create_message(session_id, "assistant".to_string(), assistant_text.clone())
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
            content: assistant_text,
            thinking_text,
            stop_reason: response.stop_reason,
            usage: response.usage,
            cost,
            model: response.model,
            provider_name: self.provider.name().to_string(),
            perf_metrics: response.perf_metrics,
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
        let (model_name, request, _message_service, _session_service) = self
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
        self.send_message_with_tools_inner(session_id, user_message, model, read_only_mode, None)
            .await
    }

    /// Streaming variant — identical to [`send_message_with_tools_and_mode`] but forwards
    /// each LLM text delta to `chunk_tx` so the TUI can render incremental output.
    pub async fn send_message_with_tools_and_mode_streaming(
        &self,
        session_id: Uuid,
        user_message: String,
        model: Option<String>,
        read_only_mode: bool,
        chunk_tx: mpsc::UnboundedSender<String>,
    ) -> Result<AgentResponse> {
        self.send_message_with_tools_inner(
            session_id,
            user_message,
            model,
            read_only_mode,
            Some(chunk_tx),
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

        let model_name = model.unwrap_or_else(|| self.provider.default_model().to_string());
        let context_window = self.provider.context_window(&model_name).unwrap_or(4096);

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
        let _user_db_msg = message_service
            .create_message(session_id, "user".to_string(), user_message)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Resolve model: router picks tier-appropriate model when available (T046)
        let model_name = if let Some(ref router) = self.model_router {
            let last_text = context
                .messages
                .last()
                .map(|m| {
                    m.content
                        .iter()
                        .filter_map(|b| {
                            if let ContentBlock::Text { text } = b {
                                Some(text.as_str())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_default();
            let tier = crate::tui::prompt_analyzer::PromptAnalyzer::new().classify_tier(&last_text);
            let (_, model_id) = router.resolve(tier);
            model_id.to_string()
        } else {
            model_name
        };

        // Create tool execution context.
        // Only wire a SubAgentLauncher for top-level services; sub-agents started by
        // AgentServiceLauncher have allow_sub_agents=false to prevent recursive spawning.
        let mut tool_context = ToolExecutionContext::new(session_id)
            .with_auto_approve(self.auto_approve_tools)
            .with_working_directory(self.working_directory.clone())
            .with_read_only_mode(read_only_mode);
        if self.allow_sub_agents {
            let launcher = Arc::new(AgentServiceLauncher::new(
                self.provider.clone(),
                self.context.clone(),
                self.tool_registry.clone(),
                self.working_directory.clone(),
                self.default_system_prompt.clone(),
            ));
            tool_context = tool_context.with_sub_agent_launcher(launcher);
        }

        // Tool execution loop
        let mut iteration = 0;
        let mut total_input_tokens = 0u32;
        let mut total_output_tokens = 0u32;
        let mut final_response: Option<LLMResponse> = None;
        let mut recent_tool_calls: Vec<String> = Vec::new(); // Track tool calls to detect loops
                                                             // Consecutive tool-calling turns in which the model said nothing to the
                                                             // user. Reset by any turn that produces text. See MAX_SILENT_TOOL_CALLS.
        let mut silent_tool_calls: usize = 0;

        while iteration < self.max_tool_iterations {
            iteration += 1;

            // Build LLM request with tools if available
            let mut request =
                LLMRequest::new(model_name.clone(), context.messages.clone()).with_max_tokens(4096);

            if let Some(system) = &context.system_prompt {
                request = request.with_system(system.clone());
            }

            // Add tools if registry has any
            let tool_count = self.tool_registry.count();
            tracing::debug!("Tool registry contains {} tools", tool_count);
            if tool_count > 0 {
                let tool_defs = self.tool_registry.get_tool_definitions();
                tracing::debug!("Adding {} tool definitions to request", tool_defs.len());
                request = request.with_tools(tool_defs);
            } else {
                tracing::warn!("No tools registered in tool registry!");
            }

            // Send to provider — stream when chunk_tx is available, otherwise block.
            let response = Self::call_provider_streaming(
                &self.provider,
                request,
                chunk_tx.as_ref(),
                &model_name,
            )
            .await
            .map_err(AgentError::Provider)?;

            // Track token usage
            total_input_tokens += response.usage.input_tokens;
            total_output_tokens += response.usage.output_tokens;

            // Accumulate prompt-cache metrics (T065)
            if let Some(ref cm) = response.cache_metrics {
                context.accumulated_cache_metrics.read_tokens += cm.read_tokens;
                context.accumulated_cache_metrics.creation_tokens += cm.creation_tokens;
            }

            // Check if response contains tool use
            tracing::debug!("Response has {} content blocks", response.content.len());
            for (i, block) in response.content.iter().enumerate() {
                match block {
                    ContentBlock::Text { text } => {
                        tracing::debug!(
                            "Block {}: Text ({}...)",
                            i,
                            &text.chars().take(50).collect::<String>()
                        );
                    }
                    ContentBlock::ToolUse { id, name, input: _ } => {
                        tracing::debug!("Block {}: ToolUse {{ name: {}, id: {} }}", i, name, id);
                    }
                    _ => {
                        tracing::debug!("Block {}: Other content block", i);
                    }
                }
            }

            let tool_uses: Vec<_> = response
                .content
                .iter()
                .filter_map(|block| {
                    if let ContentBlock::ToolUse { id, name, input } = block {
                        Some((id.clone(), name.clone(), input.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            tracing::debug!("Found {} tool uses to execute", tool_uses.len());

            if tool_uses.is_empty() {
                // No tool use - we're done
                tracing::debug!("No tool uses found, completing with final response");
                final_response = Some(response);
                break;
            }

            // Detect tool loops: Track the current batch of tool calls
            // Include arguments in signature to distinguish different calls
            // For example: ls(./src) vs ls(./src/cli) should be different
            let current_call_signature = tool_uses
                .iter()
                .map(|(_, name, input)| {
                    match name.as_str() {
                        "plan" => {
                            // Extract operation from plan tool input
                            if let Some(operation) = input.get("operation").and_then(|v| v.as_str())
                            {
                                // For add_task, include task title to distinguish different tasks
                                if operation == "add_task" {
                                    if let Some(title) = input.get("title").and_then(|v| v.as_str())
                                    {
                                        format!("{}:{}:{}", name, operation, title)
                                    } else {
                                        format!("{}:{}", name, operation)
                                    }
                                } else {
                                    format!("{}:{}", name, operation)
                                }
                            } else {
                                name.to_string()
                            }
                        }

                        // File system exploration tools - include path to distinguish calls
                        "ls" => {
                            if let Some(path) = input.get("path").and_then(|v| v.as_str()) {
                                // Normalize path separators for consistent comparison
                                let normalized = path.replace('\\', "/");
                                format!("ls:{}", normalized)
                            } else {
                                "ls:".to_string()
                            }
                        }

                        "glob" => {
                            if let Some(pattern) = input.get("pattern").and_then(|v| v.as_str()) {
                                format!("glob:{}", pattern)
                            } else {
                                "glob:".to_string()
                            }
                        }

                        "grep" => {
                            // Include pattern AND path to distinguish searches
                            let pattern =
                                input.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
                            let path = input.get("path").and_then(|v| v.as_str()).unwrap_or("");
                            format!("grep:{}:{}", pattern, path)
                        }

                        "read_file" => {
                            if let Some(path) = input.get("file_path").and_then(|v| v.as_str()) {
                                let normalized = path.replace('\\', "/");
                                format!("read_file:{}", normalized)
                            } else {
                                "read_file:".to_string()
                            }
                        }

                        // File modification tools - include file path
                        "write_file" | "edit_file" => {
                            if let Some(path) = input.get("file_path").and_then(|v| v.as_str()) {
                                let normalized = path.replace('\\', "/");
                                format!("{}:{}", name, normalized)
                            } else {
                                format!("{}:", name)
                            }
                        }

                        // Command execution - include command
                        "bash" => {
                            if let Some(cmd) = input.get("command").and_then(|v| v.as_str()) {
                                // Normalize and truncate for signature
                                let cmd_normalized = cmd.replace('\\', "/");
                                let cmd_short: String = cmd_normalized.chars().take(100).collect();
                                format!("bash:{}", cmd_short)
                            } else {
                                "bash:".to_string()
                            }
                        }

                        // Other tools: name plus an input hash, so different
                        // calls to the same tool never share a signature.
                        _ => {
                            use std::collections::hash_map::DefaultHasher;
                            use std::hash::{Hash, Hasher};
                            let mut h = DefaultHasher::new();
                            input.to_string().hash(&mut h);
                            format!("{}:{:016x}", name, h.finish())
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join(",");

            recent_tool_calls.push(current_call_signature.clone());

            // Keep only last 15 iterations for loop detection (increased for deep exploration)
            if recent_tool_calls.len() > 15 {
                recent_tool_calls.remove(0);
            }

            // Drift guard. The identical-call check below only fires when the
            // model repeats the *same* call, so a model that keeps picking
            // *different* tools sails past it and burns all 20 iterations. That
            // is the common failure: the very first `bash` returns the answer,
            // the model ignores it and wanders off through ls, glob, write_file,
            // task_manager..., ending in "Maximum tool iterations exceeded".
            //
            // Any turn where the model emits text is making progress, so the
            // counter resets. Only an unbroken run of tool calls with nothing
            // said to the user counts as drift.
            const MAX_SILENT_TOOL_CALLS: usize = 6;
            let produced_text = response
                .content
                .iter()
                .any(|b| matches!(b, ContentBlock::Text { text } if !text.trim().is_empty()));
            if produced_text {
                silent_tool_calls = 0;
            } else {
                silent_tool_calls += 1;
            }
            let drifting = silent_tool_calls >= MAX_SILENT_TOOL_CALLS;
            if drifting {
                tracing::warn!(
                    "⚠️ {} consecutive tool calls with no answer to the user \
                     (last: '{}'). Forcing a text response.",
                    silent_tool_calls,
                    current_call_signature
                );
            }

            // Check for repeated patterns with tool-specific thresholds
            // This will only trigger for truly identical calls (same tool + same arguments)

            // Determine loop threshold from the first tool's declared
            // capabilities, so the classification cannot drift from the
            // registry's actual tool names the way hardcoded strings did.
            let first_tool_caps = tool_uses
                .first()
                .and_then(|(_, name, _)| self.tool_registry.get(name))
                .map(|t| t.capabilities())
                .unwrap_or_default();
            let is_modification_tool = has_mutating_capability(&first_tool_caps);
            let is_exploration_tool = !is_modification_tool
                && first_tool_caps
                    .iter()
                    .any(|c| matches!(c, ToolCapability::ReadFiles | ToolCapability::Network));

            // Higher threshold for exploration tools (allow deep directory traversal)
            // Lower threshold for modification tools (dangerous if looping)
            let loop_threshold = if is_exploration_tool {
                10 // Allow up to 10 identical calls for exploration
            } else if is_modification_tool {
                2 // Only 2 identical calls for modification tools
            } else {
                3 // Default: 3 identical calls
            };

            // The model is repeating one identical call.
            let repeating = recent_tool_calls.len() >= loop_threshold
                && recent_tool_calls[recent_tool_calls.len() - loop_threshold..]
                    .iter()
                    .all(|call| call == &current_call_signature);

            // Either failure mode lands in the same recovery: re-ask with no
            // tools, forcing the model to answer from what it already has.
            if repeating || drifting {
                {
                    tracing::warn!(
                        "⚠️ Detected tool loop: '{}' called {} times in a row. Breaking loop.",
                        current_call_signature,
                        loop_threshold
                    );

                    if is_exploration_tool {
                        tracing::info!(
                            "💡 Hint: The model is stuck trying to access the same path {} times. \
                             This often means the path doesn't exist or the model is confused about the directory structure.",
                            loop_threshold
                        );
                    } else if is_modification_tool {
                        tracing::warn!(
                            "⚠️ Modification tool loop detected! This could be dangerous. \
                             The model tried to modify the same file/run the same command {} times.",
                            loop_threshold
                        );
                    }

                    // T052: Instead of silently breaking, inject a recovery hint and let the
                    // LLM self-correct via a follow-up turn (no tools = forced text response).
                    tracing::warn!(
                        "⚠️ Detected tool loop: '{}' called {} times in a row. Injecting recovery hint.",
                        current_call_signature,
                        loop_threshold
                    );

                    let hint = if repeating {
                        format!(
                            "⚠️ Loop detected: you called '{}' {} times in a row without making progress. \
                             Please reassess your approach. Summarise what you have found so far and \
                             explain what the next step should be without repeating the same tool call.",
                            current_call_signature, loop_threshold
                        )
                    } else {
                        format!(
                            "⚠️ You have made {} tool calls without answering the user. The results you \
                             already have are enough. Do NOT call any more tools. Answer the user's \
                             original question now, using only what those tools returned.",
                            silent_tool_calls
                        )
                    };

                    // Add the assistant's last response and a user recovery message to context.
                    let assistant_msg = Message {
                        role: crate::llm::provider::Role::Assistant,
                        content: response.content.clone(),
                    };
                    context.add_message(assistant_msg);
                    context.add_message(Message::user(hint));

                    // Ask LLM to recover without tools so it is forced to produce a text answer.
                    let recovery_request =
                        LLMRequest::new(model_name.clone(), context.messages.clone())
                            .with_max_tokens(2048);
                    let recovery_request = if let Some(system) = &context.system_prompt {
                        recovery_request.with_system(system.clone())
                    } else {
                        recovery_request
                    };

                    match self.provider.complete(recovery_request).await {
                        Ok(recovery_response) => {
                            final_response = Some(recovery_response);
                        }
                        Err(_) => {
                            final_response = Some(response);
                        }
                    }
                    break;
                }
            }

            // Execute tools and build response message.
            // Parallelizable tools (read-only, idempotent) run concurrently; others sequentially.
            // Tool results are cached per-session to avoid redundant calls (T037 / T049).
            let mut tool_results: Vec<ContentBlock> = Vec::new();

            // Partition tool uses
            let (parallel_uses, sequential_uses): (Vec<_>, Vec<_>) = tool_uses
                .into_iter()
                .partition(|(_, name, _)| is_parallelizable(name));

            // --- Parallel execution ---
            let parallel_futures: Vec<_> = parallel_uses
                .into_iter()
                .map(|(tool_id, tool_name, tool_input)| {
                    let registry = Arc::clone(&self.tool_registry);
                    let cache = Arc::clone(&self.tool_cache);
                    let ctx = tool_context.clone();
                    async move {
                        let cache_key = CacheKey::from_tool(&tool_name, &tool_input);
                        if let Some(cached) = cache.get(&cache_key) {
                            tracing::debug!("Cache hit for tool '{}'", tool_name);
                            return ContentBlock::ToolResult {
                                tool_use_id: tool_id,
                                content: cached,
                                is_error: Some(false),
                            };
                        }
                        match registry.execute(&tool_name, tool_input, &ctx).await {
                            Ok(result) => {
                                let content = if result.success {
                                    result.output
                                } else {
                                    result
                                        .error
                                        .unwrap_or_else(|| "Tool execution failed".to_string())
                                };
                                if result.success {
                                    cache.insert_for_tool(cache_key, content.clone());
                                }
                                ContentBlock::ToolResult {
                                    tool_use_id: tool_id,
                                    content,
                                    is_error: Some(!result.success),
                                }
                            }
                            Err(e) => ContentBlock::ToolResult {
                                tool_use_id: tool_id,
                                content: format!("Tool execution error: {}", e),
                                is_error: Some(true),
                            },
                        }
                    }
                })
                .collect();

            let parallel_results = join_all(parallel_futures).await;
            tool_results.extend(parallel_results);

            // --- Sequential execution (tools requiring approval or non-idempotent) ---
            // Set when a tool with a mutating capability actually executes
            // (approved or auto), so read caches can be invalidated below.
            let mut mutating_tool_ran = false;
            for (tool_id, tool_name, tool_input) in sequential_uses {
                tracing::info!(
                    "Executing tool '{}' (iteration {}/{})",
                    tool_name,
                    iteration,
                    self.max_tool_iterations
                );

                // Check if approval is needed. `requires_approval` is a static
                // property of the tool (bash always returns true), so it alone
                // would re-prompt for every `ls`. Defer to the permission policy
                // too: if it explicitly vouches for this exact command - the
                // program is on `security.allow_bash` and there is no active
                // shell operator to smuggle another one in - that stands in for
                // the user's approval. Anything not allowlisted still prompts.
                let needs_approval = if let Some(tool) = self.tool_registry.get(&tool_name) {
                    tool.requires_approval()
                        && !self.auto_approve_tools
                        && !tool_context.auto_approve
                        && !self.tool_registry.is_trusted(&tool_name, &tool_input)
                } else {
                    false
                };

                // Request approval if needed
                if needs_approval {
                    if let Some(ref approval_callback) = self.approval_callback {
                        // Get tool details for approval request
                        let tool_info = if let Some(tool) = self.tool_registry.get(&tool_name) {
                            ToolApprovalInfo {
                                tool_name: tool_name.clone(),
                                tool_description: tool.description().to_string(),
                                tool_input: tool_input.clone(),
                                capabilities: tool
                                    .capabilities()
                                    .iter()
                                    .map(|c| format!("{:?}", c))
                                    .collect(),
                            }
                        } else {
                            // Tool not found, skip approval
                            tool_results.push(ContentBlock::ToolResult {
                                tool_use_id: tool_id,
                                content: format!("Tool not found: {}", tool_name),
                                is_error: Some(true),
                            });
                            continue;
                        };

                        // Call approval callback. Log the inputs: for `bash` the
                        // command decides whether the allowlist trusts it, and
                        // without it a prompt here is impossible to explain.
                        tracing::info!(
                            "Requesting user approval for tool '{}' with input {}",
                            tool_name,
                            tool_input
                        );
                        match approval_callback(tool_info).await {
                            Ok(approved) => {
                                if !approved {
                                    tracing::warn!("User denied approval for tool '{}'", tool_name);
                                    tool_results.push(ContentBlock::ToolResult {
                                        tool_use_id: tool_id,
                                        content: "User denied permission to execute this tool"
                                            .to_string(),
                                        is_error: Some(true),
                                    });
                                    continue;
                                }
                                tracing::info!("User approved tool '{}'", tool_name);
                                // Create approved context for this tool execution
                                let approved_tool_context = ToolExecutionContext {
                                    session_id: tool_context.session_id,
                                    working_directory: tool_context.working_directory.clone(),
                                    env_vars: tool_context.env_vars.clone(),
                                    auto_approve: true, // User approved this execution
                                    timeout_secs: tool_context.timeout_secs,
                                    read_only_mode: tool_context.read_only_mode,
                                    sub_agent_launcher: tool_context.sub_agent_launcher.clone(),
                                };

                                // Execute the tool with approved context
                                mutating_tool_ran |= self
                                    .tool_registry
                                    .get(&tool_name)
                                    .is_some_and(|t| has_mutating_capability(&t.capabilities()));
                                match self
                                    .tool_registry
                                    .execute(&tool_name, tool_input, &approved_tool_context)
                                    .await
                                {
                                    Ok(result) => {
                                        tool_results.push(ContentBlock::ToolResult {
                                            tool_use_id: tool_id,
                                            content: if result.success {
                                                result.output
                                            } else {
                                                result.error.unwrap_or_else(|| {
                                                    "Tool execution failed".to_string()
                                                })
                                            },
                                            is_error: Some(!result.success),
                                        });
                                    }
                                    Err(e) => {
                                        tool_results.push(ContentBlock::ToolResult {
                                            tool_use_id: tool_id,
                                            content: format!("Tool execution error: {}", e),
                                            is_error: Some(true),
                                        });
                                    }
                                }
                                continue; // Skip the normal execution path below
                            }
                            Err(e) => {
                                tracing::error!("Approval callback error: {}", e);
                                tool_results.push(ContentBlock::ToolResult {
                                    tool_use_id: tool_id,
                                    content: format!("Approval request failed: {}", e),
                                    is_error: Some(true),
                                });
                                continue;
                            }
                        }
                    } else {
                        // No approval callback configured, deny execution
                        tracing::warn!(
                            "Tool '{}' requires approval but no approval callback configured",
                            tool_name
                        );
                        tool_results.push(ContentBlock::ToolResult {
                            tool_use_id: tool_id,
                            content: "Tool requires approval but no approval mechanism configured"
                                .to_string(),
                            is_error: Some(true),
                        });
                        continue;
                    }
                }

                // Execute the tool
                mutating_tool_ran |= self
                    .tool_registry
                    .get(&tool_name)
                    .is_some_and(|t| has_mutating_capability(&t.capabilities()));
                match self
                    .tool_registry
                    .execute(&tool_name, tool_input, &tool_context)
                    .await
                {
                    Ok(result) => {
                        tool_results.push(ContentBlock::ToolResult {
                            tool_use_id: tool_id,
                            content: if result.success {
                                result.output
                            } else {
                                result
                                    .error
                                    .unwrap_or_else(|| "Tool execution failed".to_string())
                            },
                            is_error: Some(!result.success),
                        });
                    }
                    Err(e) => {
                        tool_results.push(ContentBlock::ToolResult {
                            tool_use_id: tool_id,
                            content: format!("Tool execution error: {}", e),
                            is_error: Some(true),
                        });
                    }
                }
            }

            // A mutating tool (write_file, edit_file, bash, …) may have
            // changed the filesystem — drop cached results of every tool
            // that reads it, so the next read sees the current state
            // instead of a stale entry within its TTL. Which tools count
            // as reads comes from their declared capabilities, not a
            // hardcoded name list; unknown tools are invalidated too.
            if mutating_tool_ran {
                self.tool_cache.invalidate_matching(|tool_name| {
                    self.tool_registry.get(tool_name).map_or(true, |t| {
                        t.capabilities().contains(&ToolCapability::ReadFiles)
                    })
                });
            }

            // Add assistant message with tool use to context
            let assistant_msg = Message {
                role: crate::llm::provider::Role::Assistant,
                content: response.content.clone(),
            };
            context.add_message(assistant_msg);

            // Add user message with tool results to context
            let tool_result_msg = Message {
                role: crate::llm::provider::Role::User,
                content: tool_results,
            };
            context.add_message(tool_result_msg);

            // Trigger compaction if token budget is >80% full (T032)
            if context.should_compact() {
                if let Some(ref pool) = self.pool {
                    tracing::info!("Context at >80% token budget — triggering compaction");
                    match crate::llm::agent::compaction::compact(&mut context, pool).await {
                        Ok(record) => {
                            tracing::info!(
                                "Compaction complete: {} → {} tokens",
                                record.tokens_before,
                                record.tokens_after
                            );
                        }
                        Err(e) => {
                            tracing::warn!("Compaction failed (context unchanged): {}", e);
                        }
                    }
                } else {
                    tracing::warn!(
                        "Context at >80% token budget but no DB pool — compaction skipped"
                    );
                }
            }

            // Check if we've hit max iterations
            if iteration >= self.max_tool_iterations {
                return Err(AgentError::MaxIterationsExceeded(self.max_tool_iterations));
            }
        }

        let response = final_response.ok_or_else(|| {
            AgentError::Internal("Tool loop completed without final response".to_string())
        })?;

        // Extract text and thinking from final response
        let assistant_text = Self::extract_text_from_response(&response);
        let thinking_text = Self::extract_thinking_from_response(&response);

        // Save final assistant response to database
        let assistant_db_msg = message_service
            .create_message(session_id, "assistant".to_string(), assistant_text.clone())
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Calculate total cost
        let total_tokens = total_input_tokens + total_output_tokens;
        let cost =
            self.provider
                .calculate_cost(&response.model, total_input_tokens, total_output_tokens);

        // Update message with usage info
        message_service
            .update_message_usage(assistant_db_msg.id, total_tokens as i32, cost)
            .await
            .map_err(|e| AgentError::Database(e.to_string()))?;

        // Record which provider answered and, if available, its perf metrics.
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
            content: assistant_text,
            thinking_text,
            stop_reason: response.stop_reason,
            usage: crate::llm::provider::TokenUsage {
                input_tokens: total_input_tokens,
                output_tokens: total_output_tokens,
            },
            cost,
            model: response.model,
            provider_name: self.provider.name().to_string(),
            perf_metrics: response.perf_metrics,
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
    ) -> Result<(String, LLMRequest, MessageService, SessionService)> {
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
        let context_window = self.provider.context_window(&model_name).unwrap_or(4096);

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

        // Build base LLM request
        let request =
            LLMRequest::new(model_name.clone(), context.messages.clone()).with_max_tokens(4096);

        let request = if let Some(system) = context.system_prompt {
            request.with_system(system)
        } else {
            request
        };

        Ok((model_name, request, message_service, session_service))
    }

    /// Extract text content from an LLM response
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

/// Implements [`SubAgentLauncher`] by creating a fresh [`AgentService`] and
/// running the sub-agent prompt in a detached Tokio task.
///
/// Wired into [`ToolExecutionContext`] so the `AgentTool` can fire sub-agents
/// without depending directly on `AgentService` internals.
pub struct AgentServiceLauncher {
    provider: Arc<dyn Provider>,
    context: ServiceContext,
    tool_registry: Arc<ToolRegistry>,
    working_directory: std::path::PathBuf,
    system_prompt: Option<String>,
}

impl std::fmt::Debug for AgentServiceLauncher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentServiceLauncher")
            .field("working_directory", &self.working_directory)
            .finish_non_exhaustive()
    }
}

impl AgentServiceLauncher {
    pub fn new(
        provider: Arc<dyn Provider>,
        context: ServiceContext,
        tool_registry: Arc<ToolRegistry>,
        working_directory: std::path::PathBuf,
        system_prompt: Option<String>,
    ) -> Self {
        Self {
            provider,
            context,
            tool_registry,
            working_directory,
            system_prompt,
        }
    }
}

#[async_trait::async_trait]
impl crate::llm::tools::SubAgentLauncher for AgentServiceLauncher {
    async fn launch(
        &self,
        _agent_id: uuid::Uuid,
        description: &str,
        prompt: &str,
    ) -> std::result::Result<(), String> {
        let mut svc = AgentService::new(self.provider.clone(), self.context.clone())
            .with_tool_registry(self.tool_registry.clone())
            .with_working_directory(self.working_directory.clone())
            .with_auto_approve_tools(true)
            .with_max_tool_iterations(20)
            .with_allow_sub_agents(false);

        if let Some(sp) = &self.system_prompt {
            svc = svc.with_system_prompt(sp.clone());
        }

        let prompt = prompt.to_string();
        let description = description.to_string();
        let context = self.context.clone();
        tokio::spawn(async move {
            let session_svc = crate::services::SessionService::new(context);
            let session = match session_svc
                .create_session(Some(format!("sub-agent: {description}")))
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!(%description, error = %e, "Sub-agent failed to create session");
                    return;
                }
            };
            if let Err(e) = svc.send_message_with_tools(session.id, prompt, None).await {
                tracing::warn!(%description, error = %e, "Sub-agent failed");
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use crate::llm::provider::{LLMRequest, LLMResponse, TokenUsage};
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
}
