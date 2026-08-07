//! TUI Application State
//!
//! Core state management for the terminal user interface.

use super::events::{AppMode, EventHandler, ToolApprovalRequest, ToolApprovalResponse, TuiEvent};
use super::prompt_analyzer::PromptAnalyzer;
use crate::config::PlanExecMode;
use crate::db::models::{Message, Session};
use crate::llm::agent::AgentService;
use crate::plan::PlanDocument;
use crate::services::{MessageService, PlanService, ServiceContext, SessionService};
use anyhow::Result;
use ratatui_textarea::{CursorMove, TextArea};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Display message for UI rendering
#[derive(Debug, Clone)]
pub struct DisplayMessage {
    pub id: Uuid,
    pub role: String,
    pub content: String,
    /// Extended thinking trace, if the response included one
    pub thinking_text: Option<String>,
    /// Whether the thinking block is currently expanded in the UI
    pub thinking_expanded: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub token_count: Option<i32>,
    pub cost: Option<f64>,
    /// Name of the provider that generated this message (e.g. "ollama"),
    /// if known.
    pub provider_name: Option<String>,
    /// Runtime performance metrics (load/prefill/generation durations),
    /// if the provider exposes them.
    pub perf_metrics: Option<crate::llm::provider::PerfMetrics>,
    /// Generation throughput in tokens/second, precomputed when the message
    /// is created (needs the exact output-token count, which isn't
    /// recoverable from the combined `token_count` stored in the DB - so
    /// this is `None` again after a session reload).
    pub tokens_per_second: Option<f64>,
}

impl From<Message> for DisplayMessage {
    fn from(msg: Message) -> Self {
        let perf_metrics = msg
            .perf_metrics_json
            .as_deref()
            .and_then(|json| serde_json::from_str(json).ok());

        Self {
            id: msg.id,
            role: msg.role,
            content: msg.content,
            thinking_text: None,
            thinking_expanded: false,
            timestamp: msg.created_at,
            token_count: msg.token_count,
            cost: msg.cost,
            provider_name: msg.provider_name,
            perf_metrics,
            tokens_per_second: None,
        }
    }
}

/// Main application state
pub struct App {
    // Core state
    pub current_session: Option<Session>,
    pub messages: Vec<DisplayMessage>,
    pub sessions: Vec<Session>,

    // UI state
    pub mode: AppMode,
    /// Chat message input. A `tui-textarea` widget rather than a raw
    /// `String` so the user gets real cursor movement, mid-buffer editing,
    /// and paste-at-cursor instead of append/pop-only editing.
    pub textarea: TextArea<'static>,
    /// Previously submitted inputs, oldest first, for shell-style Up/Down recall.
    /// Kept in memory only (a session's messages are in the DB, but this also
    /// holds slash commands, which are not).
    input_history: Vec<String>,
    /// Cursor into `input_history` while browsing. `None` means "not browsing" -
    /// the textarea holds the user's own draft rather than a recalled entry.
    history_pos: Option<usize>,
    /// The draft that was in the textarea when browsing started, restored when
    /// the user presses Down past the newest entry.
    history_draft: Option<String>,
    pub scroll_offset: usize,
    pub selected_session_index: usize,
    pub should_quit: bool,
    /// Whether the terminal supports the Kitty keyboard enhancement
    /// protocol (needed to disambiguate `Shift+Enter` from plain `Enter`).
    /// Set once at startup by the runner; only affects which key hints are
    /// shown, not which keys are actually handled (`Alt+Enter` always works
    /// as a newline fallback regardless of this flag).
    pub kitty_keyboard_protocol_active: bool,
    /// Current Auto Mode level (`Interactive`/`AutoPlan`/`FullAuto`),
    /// cycled with `Shift+Tab`. Shared (`Arc<Mutex<..>>`) with the tool
    /// approval callback set up in `cli::cmd_chat`, so toggling it here
    /// takes effect on the very next tool call - off by default
    /// (`Interactive`) unless overridden by `[plan_mode].mode` in config.
    pub auto_mode: Arc<Mutex<PlanExecMode>>,
    /// Configured MCP servers' connection status, snapshotted once at
    /// startup by `cli::cmd_chat` (see `crate::mcp::McpServerStatus`).
    pub mcp_status: Vec<crate::mcp::McpServerStatus>,

    // Streaming state
    pub is_processing: bool,
    pub streaming_response: Option<String>,
    pub error_message: Option<String>,
    /// The session a `send_message` call is currently outstanding for, if
    /// any. Tracked separately from `is_processing` (which reflects only
    /// whether the *currently displayed* session has a request in flight)
    /// so that switching sessions can tell the difference between "the
    /// previous session's request is still running in the background" and
    /// "nothing is actually happening for the session now on screen" -
    /// without this, switching away and back left `is_processing`/
    /// `streaming_response` permanently stuck showing whatever state the
    /// abandoned session's request last left them in.
    processing_session: Option<Uuid>,

    // Animation state
    pub animation_frame: usize,

    // Splash screen state
    splash_shown_at: Option<std::time::Instant>,

    // Approval state
    pub pending_approval: Option<ToolApprovalRequest>,
    pub show_approval_details: bool,

    // Plan mode state
    pub current_plan: Option<PlanDocument>,
    pub plan_scroll_offset: usize,
    pub selected_task_index: Option<usize>,
    pub executing_plan: bool,

    // File picker state
    pub file_picker_files: Vec<std::path::PathBuf>,
    pub file_picker_selected: usize,
    pub file_picker_scroll_offset: usize,
    pub file_picker_current_dir: std::path::PathBuf,

    // Model download dialog state (Ctrl+D, native Ollama provider)
    pub model_download_input: String,
    pub model_download_suggestions: Vec<String>,
    pub model_download_selected: usize,
    pub model_download_installed: Vec<String>,
    pub model_download_running: bool,
    pub model_download_status: Option<String>,
    pub model_download_fraction: Option<f64>,
    model_download_task: Option<tokio::task::JoinHandle<()>>,
    /// Installed model awaiting delete confirmation ('Y'/Enter confirms,
    /// 'N'/Esc cancels back to the suggestion list).
    pub model_download_confirm_delete: Option<String>,
    /// Installed model currently being deleted, if any.
    pub model_download_deleting: Option<String>,
    model_download_delete_task: Option<tokio::task::JoinHandle<()>>,
    ollama_host: String,
    /// The `[providers.ollama]` config section, applied when the Ctrl+W
    /// switch rebuilds the provider - without it a switched-to model runs
    /// unconfigured (no per-model num_ctx/sampling, no keep_alive).
    ollama_config: Option<crate::config::OllamaProviderConfig>,

    // Provider Switch dialog state (Ctrl+W, native Ollama provider)
    pub provider_switch_models: Vec<String>,
    pub provider_switch_selected: usize,
    pub provider_switch_loading: bool,

    // Local Models dialog state (Ctrl+G, llama.cpp)
    pub llama_cpp_models: Vec<super::llama_cpp_download::LlamaCppModelSummary>,
    pub llama_cpp_selected: usize,
    pub llama_cpp_loading: bool,
    pub llama_cpp_download_input: String,
    pub llama_cpp_download_running: bool,
    pub llama_cpp_download_status: Option<String>,
    pub llama_cpp_download_fraction: Option<f64>,
    llama_cpp_download_task: Option<tokio::task::JoinHandle<()>>,
    /// Local model awaiting delete confirmation ('Y'/Enter confirms,
    /// 'N'/Esc cancels back to the list) - mirrors
    /// `model_download_confirm_delete`.
    pub llama_cpp_confirm_delete: Option<std::path::PathBuf>,
    pub llama_cpp_deleting: Option<std::path::PathBuf>,
    llama_cpp_delete_task: Option<tokio::task::JoinHandle<()>>,
    /// Set while a picked model is being loaded as the active provider
    /// (`LlamaCppProvider::new()` blocks - see `llama_cpp_download`'s
    /// module doc) - drives the "Loading model…" state.
    pub llama_cpp_switching: Option<std::path::PathBuf>,
    llama_cpp_switch_task: Option<tokio::task::JoinHandle<()>>,
    llama_cpp_pending_provider: super::llama_cpp_download::PendingProvider,
    llama_cpp_models_dir: std::path::PathBuf,
    /// `providers.llama_cpp.extra_model_paths` - additional directories the
    /// Ctrl+G dialog also scans, beyond `llama_cpp_models_dir`. Empty
    /// unless configured - see `ccguf-managment-imrpoment-plan.md` M3.
    llama_cpp_extra_model_paths: Vec<std::path::PathBuf>,
    /// Ollama's models directory, only `Some` when
    /// `providers.llama_cpp.scan_ollama_models` opted in
    /// (`ProviderConfigs::llama_cpp_ollama_models_dir()` already applies
    /// that gate) - `None` means the Ctrl+G dialog doesn't scan Ollama at
    /// all, same as before this field existed.
    llama_cpp_ollama_models_dir: Option<std::path::PathBuf>,
    /// The `[providers.llama_cpp]` config section, applied (minus
    /// `model_path`, which is always the picked file) when Ctrl+G switches
    /// models - mirrors `ollama_config`'s rationale exactly.
    llama_cpp_config: Option<crate::config::LlamaCppProviderConfig>,
    /// The `.gguf` path actually active right now, if the active provider is
    /// `llama-cpp` - used by the Model Info panel for GPU-layers/quantization
    /// details, since neither is on the generic `Provider` trait.
    llama_cpp_active_model_path: Option<std::path::PathBuf>,

    // `/skills` slash command state
    pub skills_list: Vec<crate::llm::tools::skill::SkillListing>,
    pub skills_selected: usize,

    // `/mcp` slash command state
    pub mcp_selected: usize,

    // Working directory
    pub working_directory: std::path::PathBuf,

    // Services
    agent_service: Arc<AgentService>,
    session_service: SessionService,
    message_service: MessageService,
    plan_service: PlanService,

    // Events
    event_handler: EventHandler,

    // Prompt analyzer
    prompt_analyzer: PromptAnalyzer,
}

/// Build the chat input textarea with the widget's default cursor-line
/// styling cleared. `ratatui-textarea` underlines the line the cursor is on
/// by default; in a chat input that line is (usually all of) the text being
/// typed, so everything the user wrote rendered underlined. Every place that
/// constructs the input textarea must go through this, or the underline
/// comes back on the next `clear_input()`.
fn plain_textarea() -> TextArea<'static> {
    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(ratatui::style::Style::default());
    textarea
}

/// Best-effort quantization guess for the Model Info panel - `None` when
/// this build wasn't compiled with `--features gguf-management` (on by
/// `default`) or nothing could be determined at all. Gated on
/// `gguf-management`, not `llama-cpp`, since this involves no FFI - see
/// `ccguf-managment-imrpoment-plan.md` Phase M0. Prefers the real
/// GGUF-header-parsed value (Phase M1 - precise where `general.file_type`
/// is set, coarser otherwise) over the filename-convention guess, falling
/// back to the filename only when the header itself can't be read.
#[cfg(feature = "gguf-management")]
fn quantization_hint_for_path(path: &std::path::Path) -> Option<String> {
    crate::llm::provider::gguf_metadata::read_gguf_metadata(path)
        .and_then(|m| m.quantization)
        .or_else(|| {
            crate::llm::provider::llama_cpp_models::quantization_hint_from_filename(
                &path.to_string_lossy(),
            )
        })
}

#[cfg(not(feature = "gguf-management"))]
fn quantization_hint_for_path(_path: &std::path::Path) -> Option<String> {
    None
}

/// The model's native/trained context length and whether it has an
/// embedded chat template, for the Model Info panel (Ctrl+O) - the
/// `context_length`/`has_chat_template` counterpart of
/// `quantization_hint_for_path` above, same cfg-gating rationale
/// (`ccguf-managment-imrpoment-plan.md` Phase M0/M8). A second,
/// independent header read from `quantization_hint_for_path`'s - the
/// Model Info panel is a low-frequency render, not a hot loop, so the
/// minor duplication isn't worth threading the header through both call
/// sites for.
#[cfg(feature = "gguf-management")]
fn llama_cpp_context_and_chat_template_for_path(path: &std::path::Path) -> (Option<u64>, bool) {
    match crate::llm::provider::gguf_metadata::read_gguf_metadata(path) {
        Some(m) => (m.context_length, m.has_chat_template),
        None => (None, false),
    }
}

#[cfg(not(feature = "gguf-management"))]
fn llama_cpp_context_and_chat_template_for_path(_path: &std::path::Path) -> (Option<u64>, bool) {
    (None, false)
}

impl App {
    /// Create a new app instance
    pub fn new(agent_service: Arc<AgentService>, context: ServiceContext) -> Self {
        Self {
            current_session: None,
            messages: Vec::new(),
            sessions: Vec::new(),
            mode: AppMode::Splash,
            textarea: plain_textarea(),
            input_history: Vec::new(),
            history_pos: None,
            history_draft: None,
            scroll_offset: 0,
            selected_session_index: 0,
            should_quit: false,
            kitty_keyboard_protocol_active: false,
            auto_mode: Arc::new(Mutex::new(PlanExecMode::default())),
            mcp_status: Vec::new(),
            is_processing: false,
            streaming_response: None,
            processing_session: None,
            error_message: None,
            animation_frame: 0,
            splash_shown_at: Some(std::time::Instant::now()),
            pending_approval: None,
            show_approval_details: false,
            current_plan: None,
            plan_scroll_offset: 0,
            selected_task_index: None,
            executing_plan: false,
            file_picker_files: Vec::new(),
            file_picker_selected: 0,
            file_picker_scroll_offset: 0,
            file_picker_current_dir: std::env::current_dir().unwrap_or_default(),
            model_download_input: String::new(),
            model_download_suggestions: Vec::new(),
            model_download_selected: 0,
            model_download_installed: Vec::new(),
            model_download_running: false,
            model_download_status: None,
            model_download_fraction: None,
            model_download_task: None,
            model_download_confirm_delete: None,
            model_download_deleting: None,
            model_download_delete_task: None,
            ollama_host: "http://localhost:11434".to_string(),
            ollama_config: None,
            provider_switch_models: Vec::new(),
            provider_switch_selected: 0,
            provider_switch_loading: false,
            llama_cpp_models: Vec::new(),
            llama_cpp_selected: 0,
            llama_cpp_loading: false,
            llama_cpp_download_input: String::new(),
            llama_cpp_download_running: false,
            llama_cpp_download_status: None,
            llama_cpp_download_fraction: None,
            llama_cpp_download_task: None,
            llama_cpp_confirm_delete: None,
            llama_cpp_deleting: None,
            llama_cpp_delete_task: None,
            llama_cpp_switching: None,
            llama_cpp_switch_task: None,
            llama_cpp_pending_provider: Arc::new(Mutex::new(None)),
            llama_cpp_models_dir: dirs::cache_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("crustly")
                .join("models"),
            llama_cpp_extra_model_paths: Vec::new(),
            llama_cpp_ollama_models_dir: None,
            llama_cpp_config: None,
            llama_cpp_active_model_path: None,
            skills_list: Vec::new(),
            skills_selected: 0,
            mcp_selected: 0,
            working_directory: std::env::current_dir().unwrap_or_default(),
            session_service: SessionService::new(context.clone()),
            message_service: MessageService::new(context.clone()),
            plan_service: PlanService::new(context),
            agent_service,
            event_handler: EventHandler::new(),
            prompt_analyzer: PromptAnalyzer::new(),
        }
    }

    /// Get the provider name
    pub fn provider_name(&self) -> &str {
        self.agent_service.provider_name()
    }

    /// Get the provider model
    pub fn provider_model(&self) -> &str {
        self.agent_service.provider_model()
    }

    /// Get the context window (in tokens) for the active provider/model,
    /// if known.
    pub fn provider_context_window(&self) -> Option<u32> {
        self.agent_service.provider_context_window()
    }

    /// Get the most recent assistant message, if any - used by the Model
    /// Info panel to show the last response's performance metrics.
    pub fn last_assistant_message(&self) -> Option<&DisplayMessage> {
        self.messages.iter().rev().find(|m| m.role == "assistant")
    }

    /// The chat input's full text, joining multi-line content with `\n`.
    pub fn input_text(&self) -> String {
        self.textarea.lines().join("\n")
    }

    /// Whether the chat input is empty once whitespace is trimmed (matches
    /// the old `String::trim().is_empty()` submit guard - a buffer that's
    /// just whitespace still shouldn't submit).
    fn input_is_blank(&self) -> bool {
        self.textarea
            .lines()
            .iter()
            .all(|line| line.trim().is_empty())
    }

    /// Reset the chat input to empty.
    fn clear_input(&mut self) {
        self.textarea = plain_textarea();
    }

    /// Replace the chat input's entire contents with `text` (used for the
    /// Plan Mode revision-request pre-fill, which overwrites rather than
    /// appends).
    fn set_input_text(&mut self, text: &str) {
        self.textarea = plain_textarea();
        self.textarea.insert_str(text);
    }

    /// Record a submitted input for Up/Down recall.
    ///
    /// Skips consecutive duplicates, the way a shell does - resending the same
    /// message twice should not make you press Up twice to get past it.
    fn push_input_history(&mut self, content: &str) {
        if self.input_history.last().map(String::as_str) != Some(content) {
            self.input_history.push(content.to_string());
        }
        // Any submit ends browsing: the next Up starts again from the newest.
        self.history_pos = None;
        self.history_draft = None;
    }

    /// Whether the textarea cursor is on the first / last line. Up recalls
    /// history only from the first line and Down only from the last, so that
    /// vertical cursor movement still works inside a multi-line draft (which
    /// Shift+Enter can create). This is the readline/shell convention.
    fn cursor_on_first_line(&self) -> bool {
        self.textarea.cursor().0 == 0
    }

    fn cursor_on_last_line(&self) -> bool {
        self.textarea.cursor().0 + 1 >= self.textarea.lines().len()
    }

    /// Load `entry` into the input and put the cursor at the very end, so the
    /// recalled message can be edited immediately.
    fn load_history_entry(&mut self, entry: &str) {
        self.set_input_text(entry);
        self.textarea.move_cursor(CursorMove::Bottom);
        self.textarea.move_cursor(CursorMove::End);
    }

    /// Step back through submitted messages (Up). Returns false when there is
    /// no history at all, so the caller can fall back to cursor movement.
    ///
    /// The in-progress draft is stashed on the first Up and restored by Down.
    fn history_prev(&mut self) -> bool {
        if self.input_history.is_empty() {
            return false;
        }

        let next = match self.history_pos {
            // Already at the oldest entry - stay there rather than wrap.
            Some(0) => return true,
            Some(i) => i - 1,
            None => {
                self.history_draft = Some(self.input_text());
                self.input_history.len() - 1
            }
        };

        self.history_pos = Some(next);
        let entry = self.input_history[next].clone();
        self.load_history_entry(&entry);
        true
    }

    /// Step forward through submitted messages (Down). Past the newest entry,
    /// restore the draft that was being typed when browsing began.
    ///
    /// Returns false when not browsing, so Down still moves the cursor.
    fn history_next(&mut self) -> bool {
        let Some(pos) = self.history_pos else {
            return false;
        };

        if pos + 1 < self.input_history.len() {
            self.history_pos = Some(pos + 1);
            let entry = self.input_history[pos + 1].clone();
            self.load_history_entry(&entry);
        } else {
            // Walked past the newest: back to whatever was being typed.
            self.history_pos = None;
            let draft = self.history_draft.take().unwrap_or_default();
            self.load_history_entry(&draft);
        }
        true
    }

    /// Copy the last assistant response to the system clipboard - just its
    /// last fenced code block if it has one (usually what's actually
    /// wanted), otherwise the full response text. Fails silently into
    /// `error_message` rather than panicking: headless environments and
    /// some terminals/multiplexers have no working clipboard backend.
    fn copy_last_response_to_clipboard(&mut self) {
        let Some(content) = self.last_assistant_message().map(|m| m.content.clone()) else {
            self.error_message = Some("No response to copy yet.".to_string());
            return;
        };

        let text = super::markdown::last_code_block(&content).unwrap_or(content);

        if let Err(e) = arboard::Clipboard::new().and_then(|mut cb| cb.set_text(text)) {
            self.error_message = Some(format!("Couldn't copy to clipboard: {e}"));
        }
    }

    /// Paste from the system clipboard at the cursor. An explicit fallback
    /// alongside bracketed paste (`TuiEvent::Paste`) for terminals/
    /// multiplexers where bracketed paste doesn't work.
    fn paste_from_clipboard(&mut self) {
        match arboard::Clipboard::new().and_then(|mut cb| cb.get_text()) {
            Ok(text) => {
                self.textarea.insert_str(&text);
            }
            Err(e) => {
                self.error_message = Some(format!("Couldn't read clipboard: {e}"));
            }
        }
    }

    /// Initialize the app by loading or creating a session
    pub async fn initialize(&mut self) -> Result<()> {
        // Try to load most recent session
        if let Some(session) = self.session_service.get_most_recent_session().await? {
            self.load_session(session.id).await?;
        } else {
            // Create a new session if none exists
            self.create_new_session().await?;
        }

        // Load sessions list
        self.load_sessions().await?;

        Ok(())
    }

    /// Get event handler
    pub fn event_handler(&self) -> &EventHandler {
        &self.event_handler
    }

    /// Get mutable event handler
    pub fn event_handler_mut(&mut self) -> &mut EventHandler {
        &mut self.event_handler
    }

    /// Get event sender
    pub fn event_sender(&self) -> tokio::sync::mpsc::UnboundedSender<TuiEvent> {
        self.event_handler.sender()
    }

    /// Set agent service (used to inject configured agent after app creation)
    pub fn set_agent_service(&mut self, agent_service: Arc<AgentService>) {
        self.agent_service = agent_service;
    }

    /// Set the Ollama host used by the Model Download dialog (Ctrl+D).
    /// Defaults to `http://localhost:11434` if never called.
    pub fn set_ollama_host(&mut self, host: String) {
        self.ollama_host = host;
    }

    /// Record the `[providers.ollama]` config so the Ctrl+W provider switch
    /// rebuilds providers with the same settings (per-model num_ctx/sampling,
    /// keep_alive) as the one built at startup.
    pub fn set_ollama_config(&mut self, config: crate::config::OllamaProviderConfig) {
        self.ollama_config = Some(config);
    }

    /// Record the resolved local `.gguf` models directory
    /// (`ProviderConfigs::llama_cpp_models_dir()`) the Ctrl+G dialog scans
    /// and downloads into.
    pub fn set_llama_cpp_models_dir(&mut self, dir: std::path::PathBuf) {
        self.llama_cpp_models_dir = dir;
    }

    /// Record the extra discovery sources
    /// (`ProviderConfigs::llama_cpp_extra_model_paths()`/
    /// `llama_cpp_ollama_models_dir()`) the Ctrl+G dialog also scans,
    /// beyond `llama_cpp_models_dir`. Separate from
    /// `set_llama_cpp_models_dir` since the primary directory is also used
    /// as the download target (`start_llama_cpp_download`), which these
    /// extra sources are not.
    pub fn set_llama_cpp_discovery_sources(
        &mut self,
        extra_model_paths: Vec<std::path::PathBuf>,
        ollama_models_dir: Option<std::path::PathBuf>,
    ) {
        self.llama_cpp_extra_model_paths = extra_model_paths;
        self.llama_cpp_ollama_models_dir = ollama_models_dir;
    }

    /// Record the `[providers.llama_cpp]` config (and, if the active
    /// provider is `llama-cpp` at startup, its `model_path`) so the Ctrl+G
    /// switch rebuilds providers with the same settings as the one built at
    /// startup, and the Model Info panel can show GPU-layers/quantization
    /// for the model actually running - mirrors `set_ollama_config`.
    pub fn set_llama_cpp_config(&mut self, config: crate::config::LlamaCppProviderConfig) {
        if self.provider_name() == "llama-cpp" {
            self.llama_cpp_active_model_path = Some(config.model_path.clone());
        }
        self.llama_cpp_config = Some(config);
    }

    /// GPU-layers/quantization details for the Model Info panel (Ctrl+O),
    /// when the active provider is `llama-cpp`. `None` otherwise, or if no
    /// `[providers.llama_cpp]` config was ever recorded - context size is
    /// shown separately via `provider_context_window()`, already generic.
    pub fn llama_cpp_model_details(
        &self,
    ) -> Option<super::llama_cpp_download::LlamaCppModelDetails> {
        if self.provider_name() != "llama-cpp" {
            return None;
        }
        let cfg = self.llama_cpp_config.as_ref()?;
        let model_path = self
            .llama_cpp_active_model_path
            .as_ref()
            .unwrap_or(&cfg.model_path);
        let (context_length, has_chat_template) =
            llama_cpp_context_and_chat_template_for_path(model_path);
        Some(super::llama_cpp_download::LlamaCppModelDetails {
            n_gpu_layers: cfg.n_gpu_layers,
            quantization_hint: quantization_hint_for_path(model_path),
            context_length,
            has_chat_template,
        })
    }

    /// Record whether the terminal supports the Kitty keyboard enhancement
    /// protocol, detected once at startup by the runner.
    pub fn set_kitty_keyboard_protocol_active(&mut self, active: bool) {
        self.kitty_keyboard_protocol_active = active;
    }

    /// Replace the Auto Mode shared cell with one already wired into the
    /// tool approval callback (`cli::cmd_chat`), so toggling it here in the
    /// TUI actually affects tool approval rather than talking to an
    /// isolated copy. Also seeds the starting level from config.
    pub fn set_auto_mode_state(&mut self, auto_mode: Arc<Mutex<PlanExecMode>>) {
        self.auto_mode = auto_mode;
    }

    /// Current Auto Mode level.
    pub fn auto_mode(&self) -> PlanExecMode {
        self.auto_mode
            .lock()
            .expect("auto_mode mutex poisoned")
            .clone()
    }

    /// Cycle `Interactive -> AutoPlan -> FullAuto -> Interactive`.
    fn cycle_auto_mode(&mut self) {
        let mut guard = self.auto_mode.lock().expect("auto_mode mutex poisoned");
        *guard = match *guard {
            PlanExecMode::Interactive => PlanExecMode::AutoPlan,
            PlanExecMode::AutoPlan => PlanExecMode::FullAuto,
            PlanExecMode::FullAuto => PlanExecMode::Interactive,
        };
    }

    /// Record the configured MCP servers' connection status, snapshotted
    /// once at startup by `cli::cmd_chat`, for the `/mcp` view.
    pub fn set_mcp_status(&mut self, status: Vec<crate::mcp::McpServerStatus>) {
        self.mcp_status = status;
    }

    /// Receive next event
    pub async fn next_event(&mut self) -> Option<TuiEvent> {
        self.event_handler.next().await
    }

    /// Take an already-queued event without waiting. See `EventHandler::try_next`.
    pub fn try_next_event(&mut self) -> Option<TuiEvent> {
        self.event_handler.try_next()
    }

    /// Handle an event
    pub async fn handle_event(&mut self, event: TuiEvent) -> Result<()> {
        match event {
            TuiEvent::Key(key_event) => {
                // The Kitty keyboard protocol is enabled (for Shift+Enter), and
                // under it crossterm reports Release - and Repeat - as well as
                // Press. Acting on every kind runs each handler twice per
                // keypress: Up would recall an entry on Press and then step past
                // it again on Release, so history recall appeared to do nothing.
                // Only Press is a keypress.
                if key_event.kind != crossterm::event::KeyEventKind::Press {
                    return Ok(());
                }
                self.handle_key_event(key_event).await?;
            }
            TuiEvent::Paste(text) => {
                // Bracketed paste: the whole clipboard arrives as one block, so
                // it is inserted verbatim (backslashes and all) in a single edit.
                // If this never logs while pasting, the terminal is not sending
                // bracketed paste and the text is arriving as individual key
                // events instead - which is what mangles characters.
                tracing::debug!(
                    "Bracketed paste: {} chars, {} backslashes",
                    text.chars().count(),
                    text.matches('\\').count()
                );
                // Handle paste events - only in Chat mode. Inserted at the
                // cursor position rather than blindly appended.
                if self.mode == AppMode::Chat {
                    self.textarea.insert_str(&text);
                }
            }
            TuiEvent::MessageSubmitted(content) => {
                self.send_message(content).await?;
            }
            TuiEvent::ResponseChunk(session_id, chunk) => {
                if self.event_belongs_to_current_session(session_id) {
                    self.append_streaming_chunk(chunk);
                } else {
                    tracing::debug!(
                        "Dropping response chunk for session {} - no longer the active session",
                        session_id
                    );
                }
            }
            TuiEvent::ResponseComplete(session_id, response) => {
                if self.event_belongs_to_current_session(session_id) {
                    self.complete_response(response).await?;
                } else {
                    // The request itself has genuinely finished, even
                    // though its result isn't being shown - if we don't
                    // clear this, switching back to `session_id` later
                    // would incorrectly still think a request is in
                    // flight for it.
                    if self.processing_session == Some(session_id) {
                        self.processing_session = None;
                    }
                    tracing::debug!(
                        "Dropping response for session {} - no longer the active session",
                        session_id
                    );
                }
            }
            TuiEvent::Error(session_id, error) => {
                if self.event_belongs_to_current_session(session_id) {
                    if self.executing_plan {
                        self.fail_current_plan_task(&error).await?;
                    }
                    self.show_error(error);
                } else {
                    if self.processing_session == Some(session_id) {
                        self.processing_session = None;
                    }
                    tracing::debug!(
                        "Dropping error for session {} - no longer the active session: {}",
                        session_id,
                        error
                    );
                }
            }
            TuiEvent::SwitchMode(mode) => {
                self.switch_mode(mode).await?;
            }
            TuiEvent::SelectSession(session_id) => {
                self.load_session(session_id).await?;
            }
            TuiEvent::NewSession => {
                self.create_new_session().await?;
            }
            TuiEvent::Quit => {
                self.should_quit = true;
            }
            TuiEvent::Tick => {
                // Update animation frame for spinner
                self.animation_frame = self.animation_frame.wrapping_add(1);

                // Check for approval timeout
                if let Some(ref approval_request) = self.pending_approval {
                    if approval_request.is_timed_out() {
                        tracing::warn!(
                            "Approval request {} timed out after 5 minutes",
                            approval_request.request_id
                        );

                        // Auto-deny the timed-out request
                        let response = ToolApprovalResponse {
                            request_id: approval_request.request_id,
                            approved: false,
                            reason: Some("Approval request timed out after 5 minutes".to_string()),
                        };

                        // Send response
                        let _ = approval_request.response_tx.send(response.clone());
                        let _ = self
                            .event_sender()
                            .send(TuiEvent::ToolApprovalResponse(response));

                        // Clear pending approval and return to chat
                        self.pending_approval = None;
                        self.mode = AppMode::Chat;
                        self.error_message = Some("⏱️  Approval request timed out".to_string());
                    }
                }
            }
            TuiEvent::ToolApprovalRequested(request) => {
                self.handle_approval_requested(request);
            }
            TuiEvent::ToolApprovalResponse(_response) => {
                // Response is sent via channel, just update UI state
                self.pending_approval = None;
                self.show_approval_details = false;
                self.mode = AppMode::Chat;
                // Auto-scroll to show tool execution result
                self.scroll_offset = 0;
            }
            TuiEvent::Resize(_, _) | TuiEvent::AgentProcessing => {
                // These are handled by the render loop
            }
            TuiEvent::OllamaModelsListed(models) => {
                self.model_download_installed = models;
                self.refresh_model_download_suggestions();
            }
            TuiEvent::ProviderSwitchModelsListed(models) => {
                self.provider_switch_loading = false;
                self.provider_switch_models = models;
                self.provider_switch_selected = 0;
            }
            TuiEvent::OllamaPullProgress(progress) => {
                self.model_download_fraction = progress.fraction();
                self.model_download_status = Some(progress.status);
            }
            TuiEvent::OllamaPullFinished { model, error } => {
                self.model_download_running = false;
                self.model_download_task = None;
                self.model_download_status = None;
                self.model_download_fraction = None;

                let content = match error {
                    None => format!("✅ Pulled '{}' successfully.", model),
                    Some(e) => format!("❌ Failed to pull '{}': {}", model, e),
                };
                let notification = DisplayMessage {
                    id: Uuid::new_v4(),
                    role: "system".to_string(),
                    content,
                    thinking_text: None,
                    thinking_expanded: false,
                    timestamp: chrono::Utc::now(),
                    token_count: None,
                    cost: None,
                    provider_name: None,
                    perf_metrics: None,
                    tokens_per_second: None,
                };
                self.messages.push(notification);
                self.switch_mode(AppMode::Chat).await?;

                // Refresh the installed-models list in the background so the
                // newly-pulled model shows up next time the dialog opens.
                let host = self.ollama_host.clone();
                let sender = self.event_sender();
                tokio::spawn(async move {
                    let models = super::ollama_download::fetch_installed_models(host).await;
                    let _ = sender.send(TuiEvent::OllamaModelsListed(models));
                });
            }
            TuiEvent::OllamaDeleteFinished { model, error } => {
                self.model_download_deleting = None;
                self.model_download_delete_task = None;

                let content = match &error {
                    None => format!("🗑️ Deleted '{}'.", model),
                    Some(e) => format!("❌ Failed to delete '{}': {}", model, e),
                };
                let notification = DisplayMessage {
                    id: Uuid::new_v4(),
                    role: "system".to_string(),
                    content,
                    thinking_text: None,
                    thinking_expanded: false,
                    timestamp: chrono::Utc::now(),
                    token_count: None,
                    cost: None,
                    provider_name: None,
                    perf_metrics: None,
                    tokens_per_second: None,
                };
                self.messages.push(notification);

                if error.is_none() {
                    self.model_download_installed.retain(|m| m != &model);
                    self.refresh_model_download_suggestions();
                }

                self.switch_mode(AppMode::Chat).await?;
            }
            TuiEvent::LlamaCppModelsListed(models) => {
                self.llama_cpp_loading = false;
                self.llama_cpp_models = models;
                self.llama_cpp_selected = 0;
            }
            TuiEvent::LlamaCppDownloadProgress(progress) => {
                self.llama_cpp_download_fraction = progress.fraction();
                self.llama_cpp_download_status = Some(match progress.total_bytes {
                    Some(total) => format!(
                        "{:.1} / {:.1} MB",
                        progress.bytes_downloaded as f64 / 1_048_576.0,
                        total as f64 / 1_048_576.0
                    ),
                    None => format!("{:.1} MB", progress.bytes_downloaded as f64 / 1_048_576.0),
                });
            }
            TuiEvent::LlamaCppDownloadFinished { source, error } => {
                self.llama_cpp_download_running = false;
                self.llama_cpp_download_task = None;
                self.llama_cpp_download_status = None;
                self.llama_cpp_download_fraction = None;

                let content = match &error {
                    None => format!("✅ Downloaded '{}' successfully.", source),
                    Some(e) => format!("❌ Failed to download '{}': {}", source, e),
                };
                let notification = DisplayMessage {
                    id: Uuid::new_v4(),
                    role: "system".to_string(),
                    content,
                    thinking_text: None,
                    thinking_expanded: false,
                    timestamp: chrono::Utc::now(),
                    token_count: None,
                    cost: None,
                    provider_name: None,
                    perf_metrics: None,
                    tokens_per_second: None,
                };
                self.messages.push(notification);
                self.switch_mode(AppMode::Chat).await?;

                // Refresh the local model list in the background so a
                // successful download shows up next time the dialog opens.
                let models_dir = self.llama_cpp_models_dir.clone();
                let extra_model_paths = self.llama_cpp_extra_model_paths.clone();
                let ollama_models_dir = self.llama_cpp_ollama_models_dir.clone();
                let sender = self.event_sender();
                tokio::spawn(async move {
                    let models = super::llama_cpp_download::list_local(
                        models_dir,
                        extra_model_paths,
                        ollama_models_dir,
                    )
                    .await;
                    let _ = sender.send(TuiEvent::LlamaCppModelsListed(models));
                });
            }
            TuiEvent::LlamaCppDeleteFinished { path, error } => {
                self.llama_cpp_deleting = None;
                self.llama_cpp_delete_task = None;

                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                let content = match &error {
                    None => format!("🗑️ Deleted '{}'.", name),
                    Some(e) => format!("❌ Failed to delete '{}': {}", name, e),
                };
                let notification = DisplayMessage {
                    id: Uuid::new_v4(),
                    role: "system".to_string(),
                    content,
                    thinking_text: None,
                    thinking_expanded: false,
                    timestamp: chrono::Utc::now(),
                    token_count: None,
                    cost: None,
                    provider_name: None,
                    perf_metrics: None,
                    tokens_per_second: None,
                };
                self.messages.push(notification);

                if error.is_none() {
                    self.llama_cpp_models.retain(|m| m.path != path);
                    self.llama_cpp_selected = self
                        .llama_cpp_selected
                        .min(self.llama_cpp_models.len().saturating_sub(1));
                }

                self.switch_mode(AppMode::Chat).await?;
            }
            TuiEvent::LlamaCppSwitchFinished { model_path, error } => {
                self.llama_cpp_switching = None;
                self.llama_cpp_switch_task = None;

                let name = model_path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| model_path.display().to_string());

                let content = match &error {
                    None => {
                        let provider = self
                            .llama_cpp_pending_provider
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .take();
                        match provider {
                            Some(provider) => match Arc::get_mut(&mut self.agent_service) {
                                Some(service) => {
                                    service.set_provider(provider);
                                    self.llama_cpp_active_model_path = Some(model_path.clone());
                                    if let Some(cfg) = self.llama_cpp_config.as_mut() {
                                        cfg.model_path = model_path.clone();
                                    }
                                    if let Some(session) = &mut self.current_session {
                                        session.model = Some(name.clone());
                                        session.provider = Some("llama-cpp".to_string());
                                        if let Err(e) =
                                            self.session_service.update_session(session).await
                                        {
                                            tracing::warn!(
                                                "Failed to update session after model switch: {}",
                                                e
                                            );
                                        }
                                    }
                                    format!("✅ Switched to '{}'.", name)
                                }
                                None => {
                                    "❌ Can't switch provider while a response is in progress - \
                                     try again once it finishes."
                                        .to_string()
                                }
                            },
                            None => format!(
                                "❌ Model '{}' loaded but the result was lost - try again.",
                                name
                            ),
                        }
                    }
                    Some(e) => format!("❌ Failed to load '{}': {}", name, e),
                };
                let notification = DisplayMessage {
                    id: Uuid::new_v4(),
                    role: "system".to_string(),
                    content,
                    thinking_text: None,
                    thinking_expanded: false,
                    timestamp: chrono::Utc::now(),
                    token_count: None,
                    cost: None,
                    provider_name: None,
                    perf_metrics: None,
                    tokens_per_second: None,
                };
                self.messages.push(notification);
                self.switch_mode(AppMode::Chat).await?;
            }
        }
        Ok(())
    }

    /// Handle keyboard input
    async fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;

        // DEBUG: Log key events when in Plan mode
        if matches!(self.mode, AppMode::Plan) {
            tracing::debug!(
                "🔑 Plan Mode Key: code={:?}, modifiers={:?}",
                event.code,
                event.modifiers
            );
        }

        // Global shortcuts
        if keys::is_quit(&event) {
            self.should_quit = true;
            return Ok(());
        }

        if keys::is_new_session(&event) {
            self.create_new_session().await?;
            return Ok(());
        }

        if keys::is_list_sessions(&event) {
            self.switch_mode(AppMode::Sessions).await?;
            return Ok(());
        }

        if keys::is_help(&event) {
            self.switch_mode(AppMode::Help).await?;
            return Ok(());
        }

        if keys::is_clear_session(&event) {
            self.clear_session().await?;
            return Ok(());
        }

        if keys::is_toggle_plan(&event) {
            // Toggle between Chat and Plan modes
            match self.mode {
                AppMode::Chat => {
                    // Try to load any plan (not just PendingApproval)
                    self.load_plan_for_viewing().await?;
                    // Only switch if a plan was loaded
                    if self.current_plan.is_some() {
                        self.switch_mode(AppMode::Plan).await?;
                    } else {
                        tracing::info!("No plan available to display");
                        self.error_message =
                            Some("No plan available. Create a plan first.".to_string());
                    }
                }
                AppMode::Plan => self.switch_mode(AppMode::Chat).await?,
                _ => {} // Do nothing in other modes
            }
            return Ok(());
        }

        if keys::is_toggle_auto_mode(&event) {
            self.cycle_auto_mode();
            return Ok(());
        }

        if keys::is_model_download(&event) && self.mode == AppMode::Chat {
            self.open_model_download().await?;
            return Ok(());
        }

        if keys::is_model_info(&event) && self.mode == AppMode::Chat {
            self.switch_mode(AppMode::ModelInfo).await?;
            return Ok(());
        }

        if keys::is_provider_switch(&event) && self.mode == AppMode::Chat {
            self.open_provider_switch().await?;
            return Ok(());
        }

        if keys::is_llama_cpp_models(&event) && self.mode == AppMode::Chat {
            self.open_llama_cpp_models().await?;
            return Ok(());
        }

        // Mode-specific handling
        tracing::trace!("Current mode: {:?}", self.mode);
        match self.mode {
            AppMode::Splash => {
                // Check if minimum display time (3 seconds) has elapsed
                if let Some(shown_at) = self.splash_shown_at {
                    if shown_at.elapsed() >= std::time::Duration::from_secs(3) {
                        self.splash_shown_at = None;
                        self.switch_mode(AppMode::Chat).await?;
                    }
                    // If not enough time has elapsed, ignore the key press
                }
            }
            AppMode::Chat => self.handle_chat_key(event).await?,
            AppMode::Plan => self.handle_plan_key(event).await?,
            AppMode::Sessions => self.handle_sessions_key(event).await?,
            AppMode::ToolApproval => self.handle_approval_key(event).await?,
            AppMode::FilePicker => self.handle_file_picker_key(event).await?,
            AppMode::ModelDownload => self.handle_model_download_key(event).await?,
            AppMode::ProviderSwitch => self.handle_provider_switch_key(event).await?,
            AppMode::LlamaCppModelPicker => self.handle_llama_cpp_models_key(event).await?,
            AppMode::Skills => self.handle_skills_key(event).await?,
            AppMode::Mcp => self.handle_mcp_key(event).await?,
            AppMode::Help | AppMode::Settings | AppMode::ModelInfo => {
                if keys::is_cancel(&event) {
                    self.switch_mode(AppMode::Chat).await?;
                }
            }
        }

        Ok(())
    }

    /// Handle keys in chat mode
    async fn handle_chat_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;
        use crossterm::event::{KeyCode, KeyModifiers};

        if keys::is_submit(&event) && !self.input_is_blank() {
            let content = self.input_text();
            self.clear_input();
            self.push_input_history(&content);
            if !self.try_handle_slash_command(&content).await? {
                self.send_message(content).await?;
            }
        } else if keys::is_newline(&event) {
            self.textarea.insert_newline();
        } else if keys::is_cancel(&event) {
            self.clear_input();
            self.error_message = None;
        } else if keys::is_page_up(&event) {
            // Scroll up (away from bottom) to see older messages
            self.scroll_offset = self.scroll_offset.saturating_add(10);
        } else if keys::is_page_down(&event) {
            // Scroll down (toward bottom) to see newer messages
            // When we reach 0, we're at the bottom (auto-scroll mode)
            self.scroll_offset = self.scroll_offset.saturating_sub(10);
        } else if keys::is_copy_response(&event) {
            self.copy_last_response_to_clipboard();
        } else if keys::is_paste_clipboard(&event) {
            self.paste_from_clipboard();
        } else {
            let ctrl = event.modifiers.contains(KeyModifiers::CONTROL);
            match event.code {
                // AltGr. On non-US layouts it is how you type `\`, `@`, `[`, `]`,
                // `{`, `}`, `~`, `|`, `€`... - and Windows reports it as
                // CONTROL|ALT. tui-textarea's `input_without_shortcuts` drops any
                // Char carrying CONTROL, treating it as a control key, so every
                // one of those characters was silently swallowed: typing or
                // pasting `D:\Projets` landed as `D:Projets`.
                //
                // AltGr is a text-entry modifier, not a shortcut, so insert the
                // character it produced verbatim. This must precede the '@' arm
                // below: on AZERTY, '@' is itself an AltGr character, and would
                // otherwise open the file picker instead of being typed.
                KeyCode::Char(c)
                    if event.modifiers.contains(KeyModifiers::CONTROL)
                        && event.modifiers.contains(KeyModifiers::ALT) =>
                {
                    self.textarea.insert_char(c);
                }
                KeyCode::Char('@') => {
                    // Trigger file picker mode
                    self.open_file_picker().await?;
                }
                KeyCode::Char('t') if self.textarea.is_empty() => {
                    // Toggle thinking block on the most recent assistant message
                    if let Some(msg) = self
                        .messages
                        .iter_mut()
                        .rev()
                        .find(|m| m.role == "assistant" && m.thinking_text.is_some())
                    {
                        msg.thinking_expanded = !msg.thinking_expanded;
                    }
                }
                // Cursor movement - word-wise with Ctrl, char/line-wise
                // otherwise. tui-textarea's own `input()` keymap is
                // deliberately bypassed below (via `input_without_shortcuts`)
                // because its Emacs-style Ctrl+<letter> bindings collide with
                // crustly's own global shortcuts (Ctrl+W, Ctrl+P, etc.), so
                // navigation is wired explicitly here instead.
                KeyCode::Left if ctrl => self.textarea.move_cursor(CursorMove::WordBack),
                KeyCode::Right if ctrl => self.textarea.move_cursor(CursorMove::WordForward),
                KeyCode::Left => self.textarea.move_cursor(CursorMove::Back),
                KeyCode::Right => self.textarea.move_cursor(CursorMove::Forward),
                // Shell-style history recall. Only from the first/last line, so
                // Up/Down still move the cursor inside a multi-line draft; and
                // only if there is history to recall, so they behave as before
                // on a fresh session.
                KeyCode::Up if self.cursor_on_first_line() && self.history_prev() => {}
                KeyCode::Down if self.cursor_on_last_line() && self.history_next() => {}
                KeyCode::Up => self.textarea.move_cursor(CursorMove::Up),
                KeyCode::Down => self.textarea.move_cursor(CursorMove::Down),
                KeyCode::Home => self.textarea.move_cursor(CursorMove::Head),
                KeyCode::End => self.textarea.move_cursor(CursorMove::End),
                KeyCode::Backspace if ctrl => {
                    self.textarea.delete_word();
                }
                KeyCode::Delete if ctrl => {
                    self.textarea.delete_next_word();
                }
                KeyCode::Delete => {
                    self.textarea.delete_next_char();
                }
                // Enter reaches here only when is_submit rejected it (blank
                // buffer) and is_newline didn't match either (no Shift/Alt)
                // - i.e. plain Enter on empty input, which must do nothing.
                // Without this arm it would fall to the catch-all below,
                // and tui-textarea's own `input_without_shortcuts` inserts
                // a newline for any Enter unconditionally - resurrecting
                // the exact bug fixed in Phase 1.
                KeyCode::Enter => {}
                // Plain character input, Tab, and Backspace - everything
                // else that needs no app-specific handling.
                _ => {
                    self.textarea.input_without_shortcuts(event);
                }
            }
        }

        Ok(())
    }

    /// Handle keys in sessions mode
    async fn handle_sessions_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;

        if keys::is_cancel(&event) {
            self.switch_mode(AppMode::Chat).await?;
        } else if keys::is_up(&event) {
            self.selected_session_index = self.selected_session_index.saturating_sub(1);
        } else if keys::is_down(&event) {
            self.selected_session_index =
                (self.selected_session_index + 1).min(self.sessions.len().saturating_sub(1));
        } else if keys::is_enter(&event) {
            if let Some(session) = self.sessions.get(self.selected_session_index) {
                self.load_session(session.id).await?;
                self.switch_mode(AppMode::Chat).await?;
            }
        }

        Ok(())
    }

    /// Open the `/skills` list view. Scans the filesystem synchronously
    /// (cheap - a handful of directory reads, not a network call) rather
    /// than spawning a background task the way the Ollama-backed dialogs
    /// do, then switches mode.
    async fn open_skills(&mut self) -> Result<()> {
        self.skills_list = crate::llm::tools::skill::list_skills(&self.working_directory);
        self.skills_selected = 0;
        self.switch_mode(AppMode::Skills).await
    }

    /// Handle keys in the `/skills` list view.
    async fn handle_skills_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;

        if keys::is_cancel(&event) {
            self.switch_mode(AppMode::Chat).await?;
        } else if keys::is_up(&event) {
            self.skills_selected = self.skills_selected.saturating_sub(1);
        } else if keys::is_down(&event) && !self.skills_list.is_empty() {
            self.skills_selected = (self.skills_selected + 1).min(self.skills_list.len() - 1);
        }

        Ok(())
    }

    /// Open the `/mcp` list view. Shows the status snapshot taken once at
    /// startup (`App::mcp_status`, set via `set_mcp_status`) rather than
    /// reconnecting live - see the "Open Decisions" note in
    /// ergonomy-improvment.md about deferring live reconnect-on-open.
    async fn open_mcp(&mut self) -> Result<()> {
        self.mcp_selected = 0;
        self.switch_mode(AppMode::Mcp).await
    }

    /// Handle keys in the `/mcp` list view.
    async fn handle_mcp_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;

        if keys::is_cancel(&event) {
            self.switch_mode(AppMode::Chat).await?;
        } else if keys::is_up(&event) {
            self.mcp_selected = self.mcp_selected.saturating_sub(1);
        } else if keys::is_down(&event) && !self.mcp_status.is_empty() {
            self.mcp_selected = (self.mcp_selected + 1).min(self.mcp_status.len() - 1);
        }

        Ok(())
    }

    /// Try to interpret `content` (the just-submitted chat input) as a
    /// slash command. Returns `true` if it was recognized and handled (the
    /// caller must NOT also forward it to the LLM), `false` otherwise - an
    /// unrecognized `/word` (e.g. a file path pasted into the input) falls
    /// through to a normal chat message, exactly as before this existed.
    async fn try_handle_slash_command(&mut self, content: &str) -> Result<bool> {
        let trimmed = content.trim();
        if !trimmed.starts_with('/') {
            return Ok(false);
        }
        let command = trimmed.split_whitespace().next().unwrap_or(trimmed);
        match command {
            "/skills" => {
                self.open_skills().await?;
                Ok(true)
            }
            "/mcp" => {
                self.open_mcp().await?;
                Ok(true)
            }
            "/help" => {
                self.switch_mode(AppMode::Help).await?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Handle keys in plan mode
    async fn handle_plan_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;
        use crossterm::event::{KeyCode, KeyModifiers};

        // Cancel/Escape - return to chat
        if keys::is_cancel(&event) {
            self.switch_mode(AppMode::Chat).await?;
            return Ok(());
        }

        // Ctrl+A - Approve plan
        if event.code == KeyCode::Char('a') && event.modifiers.contains(KeyModifiers::CONTROL) {
            tracing::info!("✅ Ctrl+A pressed - Approving plan");
            if let Some(plan) = &mut self.current_plan {
                plan.approve();
                plan.start_execution();

                // Export plan to markdown file
                self.export_plan_to_markdown("PLAN.md").await?;

                // Save plan to file
                self.save_plan().await?;
                self.switch_mode(AppMode::Chat).await?;
                // Start executing tasks sequentially
                self.execute_plan_tasks().await?;
            }
            return Ok(());
        }

        // Ctrl+R - Reject plan
        if event.code == KeyCode::Char('r') && event.modifiers.contains(KeyModifiers::CONTROL) {
            tracing::info!("❌ Ctrl+R pressed - Rejecting plan");
            if let Some(plan) = &mut self.current_plan {
                plan.reject();
                // Save plan to file
                self.save_plan().await?;
                // Clear the plan from memory and return to chat
                self.current_plan = None;
                self.switch_mode(AppMode::Chat).await?;
            }
            return Ok(());
        }

        // Ctrl+I - Request plan revision
        if event.code == KeyCode::Char('i') && event.modifiers.contains(KeyModifiers::CONTROL) {
            tracing::info!("🔄 Ctrl+I pressed - Requesting plan revision");
            if let Some(plan) = &self.current_plan {
                // Build plan summary for context
                let plan_summary = format!(
                    "Current plan '{}' has {} tasks:\n{}",
                    plan.title,
                    plan.tasks.len(),
                    plan.tasks
                        .iter()
                        .enumerate()
                        .map(|(i, t)| format!("  {}. {} ({})", i + 1, t.title, t.task_type))
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                // Switch back to chat mode
                self.switch_mode(AppMode::Chat).await?;

                // Pre-fill input with revision request
                self.set_input_text(&format!(
                    "Please revise this plan:\n\n{}\n\nRequested changes: ",
                    plan_summary
                ));

                // Keep plan in memory for reference (don't clear it)
            }
            return Ok(());
        }

        // Arrow keys for scrolling tasks
        match event.code {
            KeyCode::Up => {
                self.plan_scroll_offset = self.plan_scroll_offset.saturating_sub(1);
            }
            KeyCode::Down => {
                if let Some(plan) = &self.current_plan {
                    let max_scroll = plan.tasks.len().saturating_sub(1);
                    self.plan_scroll_offset = (self.plan_scroll_offset + 1).min(max_scroll);
                }
            }
            KeyCode::PageUp => {
                self.plan_scroll_offset = self.plan_scroll_offset.saturating_sub(10);
            }
            KeyCode::PageDown => {
                if let Some(plan) = &self.current_plan {
                    let max_scroll = plan.tasks.len().saturating_sub(1);
                    self.plan_scroll_offset = (self.plan_scroll_offset + 10).min(max_scroll);
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Create a new session
    async fn create_new_session(&mut self) -> Result<()> {
        let mut session = self
            .session_service
            .create_session(Some("New Chat".to_string()))
            .await?;

        // Stamp the model/provider now. They were previously only filled in
        // after the first assistant reply landed, so a fresh session rendered
        // its header as "unknown" until then - even though the provider was
        // known all along.
        session.model = Some(self.agent_service.provider_model().to_string());
        session.provider = Some(self.agent_service.provider_name().to_string());
        if let Err(e) = self.session_service.update_session(&session).await {
            tracing::warn!("Failed to stamp session model/provider: {}", e);
        }

        self.current_session = Some(session.clone());
        self.messages.clear();
        self.scroll_offset = 0;
        self.mode = AppMode::Chat;
        self.sync_processing_state_for_current_session();

        // Reload sessions list
        self.load_sessions().await?;

        Ok(())
    }

    /// Load a session and its messages
    async fn load_session(&mut self, session_id: Uuid) -> Result<()> {
        let mut session = self
            .session_service
            .get_session(session_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        let messages = self
            .message_service
            .list_messages_for_session(session_id)
            .await?;

        // Sessions created before the model was recorded (and any created
        // outside the TUI) carry a NULL model, which the header renders as
        // "unknown". Fill it in from the active provider.
        if session.model.is_none() {
            session.model = Some(self.agent_service.provider_model().to_string());
            session.provider = Some(self.agent_service.provider_name().to_string());
            if let Err(e) = self.session_service.update_session(&session).await {
                tracing::warn!("Failed to stamp loaded session model/provider: {}", e);
            }
        }

        self.current_session = Some(session);
        self.messages = messages.into_iter().map(DisplayMessage::from).collect();
        self.scroll_offset = 0;
        self.sync_processing_state_for_current_session();

        Ok(())
    }

    /// Load all sessions
    async fn load_sessions(&mut self) -> Result<()> {
        use crate::db::repository::SessionListOptions;

        self.sessions = self
            .session_service
            .list_sessions(SessionListOptions {
                include_archived: false,
                limit: Some(100),
                offset: 0,
            })
            .await?;

        Ok(())
    }

    /// Clear all messages from the current session
    async fn clear_session(&mut self) -> Result<()> {
        // Refuse to clear while a response is still generating for THIS session.
        // The agent call runs as a detached background task that, on completion,
        // creates an assistant message and then updates it (usage/metrics). If we
        // delete every message for the session out from under that task, its
        // trailing `update_message_usage` finds nothing and fails with
        // "Message not found" - and a message created *after* the delete would
        // reappear as an orphan in the just-cleared session. Mirror the
        // concurrency guard `send_message` already uses for a second submission.
        if self.is_processing
            && self.processing_session == self.current_session.as_ref().map(|s| s.id)
        {
            self.error_message =
                Some("⏳ Wait for the response to finish before clearing.".to_string());
            return Ok(());
        }

        if let Some(session) = &self.current_session {
            // Delete all messages from the database
            self.message_service
                .delete_messages_for_session(session.id)
                .await?;

            // Clear messages from UI
            self.messages.clear();
            self.scroll_offset = 0;
            self.streaming_response = None;
            self.error_message = None;
        }

        Ok(())
    }

    /// Send a message to the agent
    async fn send_message(&mut self, content: String) -> Result<()> {
        // Guard against a second concurrent request against the same session:
        // without this, submitting again while a response is still streaming
        // spawns a duplicate `send_message_with_tools_and_mode_streaming` call
        // that races the first for message ordering, DB writes, and (during
        // plan execution) task-completion bookkeeping.
        if self.is_processing
            && self.processing_session == self.current_session.as_ref().map(|s| s.id)
        {
            return Ok(());
        }

        if let Some(session) = &self.current_session {
            self.is_processing = true;
            self.processing_session = Some(session.id);
            self.error_message = None;

            // Analyze and transform the prompt before sending to agent
            let transformed_content = self.prompt_analyzer.analyze_and_transform(&content);

            // Log if the prompt was transformed
            if transformed_content != content {
                tracing::info!("✨ Prompt transformed with tool hints");
            }

            // Add user message to UI immediately (show original content)
            let user_msg = DisplayMessage {
                id: Uuid::new_v4(),
                role: "user".to_string(),
                content: content.clone(),
                thinking_text: None,
                thinking_expanded: false,
                timestamp: chrono::Utc::now(),
                token_count: None,
                cost: None,
                provider_name: None,
                perf_metrics: None,
                tokens_per_second: None,
            };
            self.messages.push(user_msg);

            // Auto-scroll to show the new user message
            self.scroll_offset = 0;

            // Send transformed content to agent in background with live streaming.
            let agent_service = self.agent_service.clone();
            let session_id = session.id;
            let event_sender = self.event_sender();
            let read_only_mode = self.mode == AppMode::Plan;

            let (chunk_tx, mut chunk_rx) = tokio::sync::mpsc::unbounded_channel::<String>();

            // Forward stream chunks to the TUI event loop.
            let event_sender_chunks = event_sender.clone();
            let forwarder_handle = tokio::spawn(async move {
                while let Some(chunk) = chunk_rx.recv().await {
                    let _ = event_sender_chunks.send(TuiEvent::ResponseChunk(session_id, chunk));
                }
            });

            tokio::spawn(async move {
                let result = agent_service
                    .send_message_with_tools_and_mode_streaming(
                        session_id,
                        transformed_content,
                        None,
                        read_only_mode,
                        chunk_tx,
                    )
                    .await;

                // Wait for the forwarder to drain all buffered chunks before sending
                // ResponseComplete. This guarantees ResponseComplete is always processed
                // after every ResponseChunk event, preventing the TUI from clearing
                // streaming_response while chunks are still in-flight.
                let _ = forwarder_handle.await;

                match result {
                    Ok(response) => {
                        let _ = event_sender.send(TuiEvent::ResponseComplete(session_id, response));
                    }
                    Err(e) => {
                        let _ = event_sender.send(TuiEvent::Error(session_id, e.to_string()));
                    }
                }
            });
        }

        Ok(())
    }

    /// Whether `session_id` (the session an in-flight `send_message` call
    /// was made against) is still the session on screen. `send_message`
    /// spawns the agent call as a detached background task with no
    /// cancellation; if the user switches sessions (or fires off a second
    /// request) before it resolves, its `ResponseChunk`/`ResponseComplete`/
    /// `Error` events must be dropped rather than mutating whatever session
    /// happens to be current when they arrive - otherwise one session's
    /// streamed reply gets appended to a different session's transcript.
    fn event_belongs_to_current_session(&self, session_id: Uuid) -> bool {
        self.current_session
            .as_ref()
            .is_some_and(|s| s.id == session_id)
    }

    /// Recompute `is_processing`/`streaming_response` for whichever session
    /// just became current.
    ///
    /// Call this after every `self.current_session = Some(...)` assignment
    /// (`create_new_session`, `load_session`). Without it, switching away
    /// from a session mid-request left `is_processing` stuck `true` and
    /// `streaming_response` frozen on the abandoned session's partial reply
    /// forever: `complete_response`/`show_error` (the only two places that
    /// used to reset them) are only ever reached for the *current*
    /// session's own completion, and the stale-session events that arrive
    /// afterward are correctly dropped by `event_belongs_to_current_session`
    /// without ever running either. `processing_session` tracks which
    /// session (if any) genuinely still has a request in flight, so this
    /// can tell "switched to a session with nothing happening" (the common
    /// case: reset both) apart from "switched back to a session whose
    /// request is still running" (keep showing the spinner, but the
    /// partial text that arrived while away was never accumulated, so
    /// `streaming_response` restarts empty rather than showing something
    /// wrong).
    fn sync_processing_state_for_current_session(&mut self) {
        let current_id = self.current_session.as_ref().map(|s| s.id);
        self.is_processing = current_id.is_some() && self.processing_session == current_id;
        if !self.is_processing {
            self.streaming_response = None;
        }
    }

    /// Append a streaming chunk
    fn append_streaming_chunk(&mut self, chunk: String) {
        if let Some(ref mut response) = self.streaming_response {
            response.push_str(&chunk);
        } else {
            self.streaming_response = Some(chunk);
            // Auto-scroll when response starts streaming
            self.scroll_offset = 0;
        }
    }

    /// Complete the streaming response
    async fn complete_response(
        &mut self,
        response: crate::llm::agent::AgentResponse,
    ) -> Result<()> {
        self.is_processing = false;
        self.streaming_response = None;
        self.processing_session = None;

        // Check task completion FIRST (before moving response.content)
        let task_failed = if self.executing_plan {
            self.check_task_completion(&response.content).await?
        } else {
            false
        };

        // Add assistant message to UI
        let assistant_msg = DisplayMessage {
            id: response.message_id,
            role: "assistant".to_string(),
            content: response.content,
            thinking_text: response.thinking_text,
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: Some(
                response.usage.input_tokens as i32 + response.usage.output_tokens as i32,
            ),
            cost: Some(response.cost),
            provider_name: Some(response.provider_name.clone()),
            tokens_per_second: response
                .perf_metrics
                .as_ref()
                .and_then(|pm| pm.tokens_per_second(response.usage.output_tokens)),
            perf_metrics: response.perf_metrics.clone(),
        };
        self.messages.push(assistant_msg);

        // Update session model/provider if not already set
        if let Some(session) = &mut self.current_session {
            let mut needs_save = false;
            if session.model.is_none() {
                session.model = Some(response.model.clone());
                needs_save = true;
            }
            if session.provider.is_none() {
                session.provider = Some(response.provider_name.clone());
                needs_save = true;
            }
            if needs_save {
                // Save the updated session to database
                if let Err(e) = self.session_service.update_session(session).await {
                    tracing::warn!("Failed to update session model/provider: {}", e);
                }
            }
        }

        // Auto-scroll to bottom
        self.scroll_offset = 0;

        // Handle plan execution
        if self.executing_plan {
            if task_failed {
                // Stop execution on failure
                self.executing_plan = false;
                let error_msg = DisplayMessage {
                    id: uuid::Uuid::new_v4(),
                    role: "system".to_string(),
                    content: "⚠️ Plan execution stopped due to task failure. \
                             Review the error above and decide how to proceed."
                        .to_string(),
                    thinking_text: None,
                    thinking_expanded: false,
                    timestamp: chrono::Utc::now(),
                    token_count: None,
                    cost: None,
                    provider_name: None,
                    perf_metrics: None,
                    tokens_per_second: None,
                };
                self.messages.push(error_msg);
            } else {
                // Execute next task if current one succeeded
                self.execute_next_plan_task().await?;
            }
        } else {
            // Check if a plan was created/finalized
            self.check_and_load_plan().await?;
        }

        Ok(())
    }

    /// Check if the current task completed successfully or failed
    /// Returns true if task failed, false if succeeded
    async fn check_task_completion(&mut self, response_content: &str) -> Result<bool> {
        let Some(plan) = &mut self.current_plan else {
            return Ok(false);
        };

        // Find the in-progress task
        let task_result = plan
            .tasks
            .iter_mut()
            .find(|t| matches!(t.status, crate::plan::TaskStatus::InProgress))
            .map(|task| {
                // Check for error indicators in the response
                let response_lower = response_content.to_lowercase();
                let has_error = response_lower.contains("error:")
                    || response_lower.contains("failed to")
                    || response_lower.contains("cannot")
                    || response_lower.contains("unable to")
                    || response_lower.contains("fatal:")
                    || (response_lower.contains("error") && response_lower.contains("executing"))
                    || response_lower.contains("compilation error")
                    || response_lower.contains("build failed");

                if has_error {
                    // Mark task as failed
                    task.status = crate::plan::TaskStatus::Failed;
                    task.notes = Some(
                        "Task failed during execution. Error detected in response.".to_string(),
                    );
                    true // Task failed
                } else {
                    // Mark task as completed successfully
                    task.status = crate::plan::TaskStatus::Completed;
                    task.completed_at = Some(chrono::Utc::now());
                    task.notes = Some("Task completed successfully".to_string());
                    false // Task succeeded
                }
            });

        // Save updated plan
        self.save_plan().await?;

        Ok(task_result.unwrap_or(false))
    }

    /// Load plan for manual viewing (Ctrl+P)
    /// Loads ANY plan (Draft, PendingApproval, etc.) for viewing
    async fn load_plan_for_viewing(&mut self) -> Result<()> {
        // Get session ID for session-scoped operations
        let session_id = match &self.current_session {
            Some(session) => session.id,
            None => {
                tracing::debug!("No current session, skipping plan load");
                return Ok(());
            }
        };

        tracing::debug!("Loading plan for viewing (session: {})", session_id);

        // Try loading from database first
        match self.plan_service.get_most_recent_plan(session_id).await {
            Ok(Some(plan)) => {
                tracing::info!(
                    "✅ Loaded plan from database: '{}' ({:?}, {} tasks)",
                    plan.title,
                    plan.status,
                    plan.tasks.len()
                );
                self.current_plan = Some(plan);
                return Ok(());
            }
            Ok(None) => {
                tracing::debug!("No plan found in database, checking JSON file");
            }
            Err(e) => {
                tracing::warn!("Failed to load plan from database: {}", e);
            }
        }

        // Fallback to JSON file for backward compatibility / migration
        let plan_filename = format!(".crustly_plan_{}.json", session_id);
        let plan_file = self.working_directory.join(&plan_filename);

        tracing::debug!("Looking for plan file at: {}", plan_file.display());

        match tokio::fs::read_to_string(&plan_file).await {
            Ok(content) => {
                tracing::debug!("Found plan JSON file, parsing...");
                match serde_json::from_str::<crate::plan::PlanDocument>(&content) {
                    Ok(plan) => {
                        tracing::info!(
                            "✅ Loaded plan from JSON: '{}' ({:?}, {} tasks)",
                            plan.title,
                            plan.status,
                            plan.tasks.len()
                        );

                        // Migrate to database
                        if let Err(e) = self.plan_service.create(&plan).await {
                            tracing::warn!("Failed to migrate plan to database: {}", e);
                        }

                        self.current_plan = Some(plan);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse plan JSON: {}", e);
                    }
                }
            }
            Err(_) => {
                tracing::debug!("No plan file found");
            }
        }

        Ok(())
    }

    /// Check for and load a plan if one was created
    /// Loads from database first, with JSON fallback for migration
    /// Only loads plans with status PendingApproval (for automatic notification)
    async fn check_and_load_plan(&mut self) -> Result<()> {
        // Get session ID for session-scoped operations
        let session_id = match &self.current_session {
            Some(session) => session.id,
            None => {
                tracing::debug!("No current session, skipping plan load");
                return Ok(());
            }
        };

        tracing::debug!("Checking for pending plan (session: {})", session_id);

        // Try loading from database first
        match self.plan_service.get_most_recent_plan(session_id).await {
            Ok(Some(plan)) => {
                tracing::debug!(
                    "Found plan in database: id={}, status={:?}",
                    plan.id,
                    plan.status
                );
                // Only load if plan is pending approval
                if plan.status == crate::plan::PlanStatus::PendingApproval {
                    tracing::info!("✅ Plan ready for review!");

                    // Only load if not already loaded (avoid duplicate messages)
                    if self.current_plan.is_none() {
                        let plan_title = plan.title.clone();
                        let task_count = plan.tasks.len();
                        self.current_plan = Some(plan);

                        // Add notification message to chat (stay in current mode)
                        let notification = DisplayMessage {
                            id: Uuid::new_v4(),
                            role: "system".to_string(),
                            content: format!(
                                "✅ Plan '{}' is ready!\n\n\
                                 {} tasks • Press Ctrl+P to review\n\n\
                                 Actions:\n\
                                 • Ctrl+A: Approve and execute\n\
                                 • Ctrl+R: Reject\n\
                                 • Ctrl+I: Request changes\n\
                                 • Ctrl+P: View plan",
                                plan_title, task_count
                            ),
                            thinking_text: None,
                            thinking_expanded: false,
                            timestamp: chrono::Utc::now(),
                            token_count: None,
                            cost: None,
                            provider_name: None,
                            perf_metrics: None,
                            tokens_per_second: None,
                        };

                        self.messages.push(notification);
                    }
                }
                return Ok(());
            }
            Ok(None) => {
                tracing::debug!("No pending plan found in database, checking JSON file");
            }
            Err(e) => {
                tracing::warn!("Failed to load plan from database: {}", e);
            }
        }

        // Fallback to JSON file for backward compatibility / migration
        let plan_filename = format!(".crustly_plan_{}.json", session_id);
        let plan_file = self.working_directory.join(&plan_filename);

        tracing::debug!("Looking for plan file at: {}", plan_file.display());

        // Check if file exists before trying to read
        let file_exists = plan_file.exists();
        tracing::debug!("Plan file exists: {}", file_exists);

        match tokio::fs::read_to_string(&plan_file).await {
            Ok(content) => {
                tracing::debug!("Found plan JSON file, parsing...");
                match serde_json::from_str::<crate::plan::PlanDocument>(&content) {
                    Ok(plan) => {
                        tracing::debug!(
                            "Parsed plan: id={}, status={:?}, tasks={}",
                            plan.id,
                            plan.status,
                            plan.tasks.len()
                        );
                        // Only load if plan is pending approval
                        if plan.status == crate::plan::PlanStatus::PendingApproval {
                            tracing::info!("✅ Plan ready for review!");

                            // Migrate to database
                            if let Err(e) = self.plan_service.create(&plan).await {
                                tracing::warn!("Failed to migrate plan to database: {}", e);
                            }

                            // Only load if not already loaded (avoid duplicate messages)
                            if self.current_plan.is_none() {
                                let plan_title = plan.title.clone();
                                let task_count = plan.tasks.len();
                                self.current_plan = Some(plan);

                                // Add notification message to chat (stay in current mode)
                                let notification = DisplayMessage {
                                    id: Uuid::new_v4(),
                                    role: "system".to_string(),
                                    content: format!(
                                        "✅ Plan '{}' is ready!\n\n\
                                         {} tasks • Press Ctrl+P to review\n\n\
                                         Actions:\n\
                                         • Ctrl+A: Approve and execute\n\
                                         • Ctrl+R: Reject\n\
                                         • Ctrl+I: Request changes\n\
                                         • Ctrl+P: View plan",
                                        plan_title, task_count
                                    ),
                                    thinking_text: None,
                                    thinking_expanded: false,
                                    timestamp: chrono::Utc::now(),
                                    token_count: None,
                                    cost: None,
                                    provider_name: None,
                                    perf_metrics: None,
                                    tokens_per_second: None,
                                };

                                self.messages.push(notification);
                            }
                        } else {
                            tracing::debug!(
                                "Plan status is {:?}, not PendingApproval - skipping",
                                plan.status
                            );
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse plan JSON: {}", e);
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::debug!("Plan file not found (this is normal if no plan was created)");
            }
            Err(e) => {
                tracing::warn!("Failed to read plan JSON file: {}", e);
            }
        }

        Ok(())
    }

    /// Save the current plan
    /// Dual-write: database as primary, JSON as backup
    /// Export plan to markdown file
    async fn export_plan_to_markdown(&self, filename: &str) -> Result<()> {
        if let Some(plan) = &self.current_plan {
            // Generate markdown content
            let mut markdown = String::new();
            markdown.push_str(&format!("# {}\n\n", plan.title));
            markdown.push_str(&format!("{}\n\n", plan.description));

            if !plan.context.is_empty() {
                markdown.push_str("## Context\n\n");
                markdown.push_str(&format!("{}\n\n", plan.context));
            }

            if !plan.risks.is_empty() {
                markdown.push_str("## Risks & Considerations\n\n");
                for risk in &plan.risks {
                    markdown.push_str(&format!("- {}\n", risk));
                }
                markdown.push('\n');
            }

            markdown.push_str("## Tasks\n\n");

            for task in &plan.tasks {
                markdown.push_str(&format!("### Task {}: {}\n\n", task.order, task.title));
                markdown.push_str(&format!(
                    "**Type:** {:?} | **Complexity:** {}★\n\n",
                    task.task_type, task.complexity
                ));

                if !task.dependencies.is_empty() {
                    let dep_orders: Vec<String> = task
                        .dependencies
                        .iter()
                        .filter_map(|dep_id| {
                            plan.tasks
                                .iter()
                                .find(|t| &t.id == dep_id)
                                .map(|t| t.order.to_string())
                        })
                        .collect();
                    markdown.push_str(&format!(
                        "**Dependencies:** Task(s) {}\n\n",
                        dep_orders.join(", ")
                    ));
                }

                markdown.push_str("**Implementation Steps:**\n\n");
                markdown.push_str(&format!("{}\n\n", task.description));
                markdown.push_str("---\n\n");
            }

            // UTC in the domain model -> local for display.
            markdown.push_str(&format!(
                "\n*Plan created: {}*\n",
                plan.created_at
                    .with_timezone(&chrono::Local)
                    .format("%Y-%m-%d %H:%M:%S")
            ));
            markdown.push_str(&format!(
                "*Last updated: {}*\n",
                plan.updated_at
                    .with_timezone(&chrono::Local)
                    .format("%Y-%m-%d %H:%M:%S")
            ));

            // Write markdown file to working directory
            let output_path = self.working_directory.join(filename);

            // Write markdown file (overwrite if exists)
            tokio::fs::write(&output_path, markdown)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to write markdown file: {}", e))?;

            tracing::info!("Exported plan to {}", output_path.display());
        }

        Ok(())
    }

    async fn save_plan(&self) -> Result<()> {
        if let Some(plan) = &self.current_plan {
            // Get session ID for session-scoped operations
            let session_id = match &self.current_session {
                Some(session) => session.id,
                None => {
                    tracing::warn!("Cannot save plan: no current session");
                    return Ok(());
                }
            };

            // Primary: Save to database
            // Try to update first (plan may already exist)
            match self.plan_service.update(plan).await {
                Ok(_) => {
                    tracing::debug!("Updated plan in database: {}", plan.id);
                }
                Err(_) => {
                    // If update fails, try creating (plan doesn't exist yet)
                    if let Err(e) = self.plan_service.create(plan).await {
                        tracing::error!("Failed to save plan to database: {}", e);
                        // Continue to JSON backup even if database fails
                    } else {
                        tracing::debug!("Created plan in database: {}", plan.id);
                    }
                }
            }

            // Backup: Save to JSON file (for backward compatibility and backup)
            let plan_filename = format!(".crustly_plan_{}.json", session_id);
            let plan_file = self.working_directory.join(&plan_filename);

            if let Err(e) = self.plan_service.export_to_json(plan, &plan_file).await {
                tracing::warn!("Failed to save plan JSON backup: {}", e);
            }
        }
        Ok(())
    }

    /// Execute plan tasks sequentially
    async fn execute_plan_tasks(&mut self) -> Result<()> {
        self.executing_plan = true;
        self.execute_next_plan_task().await
    }

    /// Execute the next pending task in the plan
    async fn execute_next_plan_task(&mut self) -> Result<()> {
        // Collect necessary data from plan first to avoid borrow issues
        let (task_message, completion_data) = {
            let Some(plan) = &mut self.current_plan else {
                self.executing_plan = false;
                return Ok(());
            };

            // Get tasks in dependency order
            let Some(ordered_tasks) = plan.tasks_in_order() else {
                self.executing_plan = false;
                self.show_error(
                    "❌ Cannot Execute Plan\n\n\
                     Circular dependency detected in task graph. Tasks cannot be ordered \
                     because they form a dependency cycle.\n\n\
                     💡 Fix: Review task dependencies and remove circular references.\n\
                     You can reject this plan (Ctrl+R) and ask the AI to revise it."
                        .to_string(),
                );
                return Ok(());
            };

            // Find the next pending task and extract its data
            let next_task_data = ordered_tasks
                .iter()
                .find(|task| matches!(task.status, crate::plan::TaskStatus::Pending))
                .map(|task| {
                    (
                        task.id,
                        task.order,
                        task.title.clone(),
                        task.description.clone(),
                    )
                });

            let total_tasks = plan.tasks.len();

            // Drop the immutable borrow of ordered_tasks
            drop(ordered_tasks);

            match next_task_data {
                Some((task_id, order, title, description)) => {
                    // Mark task as in progress
                    if let Some(task_mut) = plan.tasks.iter_mut().find(|t| t.id == task_id) {
                        task_mut.status = crate::plan::TaskStatus::InProgress;
                    }

                    // Prepare task message
                    let message = format!(
                        "📋 Executing Plan Task #{}/{}\n\n\
                         **{}**\n\n\
                         {}\n\n\
                         Please complete this task.",
                        order, total_tasks, title, description
                    );

                    (Some(message), None)
                }
                None => {
                    // No more pending tasks - plan is complete
                    let title = plan.title.clone();
                    let task_count = plan.tasks.len();
                    plan.complete();
                    self.executing_plan = false;

                    (None, Some((title, task_count)))
                }
            }
        };

        // Save plan after releasing borrow
        self.save_plan().await?;

        // Handle results
        if let Some((title, task_count)) = completion_data {
            // Add completion message
            let completion_msg = DisplayMessage {
                id: uuid::Uuid::new_v4(),
                role: "system".to_string(),
                content: format!(
                    "✅ Plan '{}' completed successfully!\n\
                     All {} tasks have been executed.",
                    title, task_count
                ),
                thinking_text: None,
                thinking_expanded: false,
                timestamp: chrono::Utc::now(),
                token_count: None,
                cost: None,
                provider_name: None,
                perf_metrics: None,
                tokens_per_second: None,
            };
            self.messages.push(completion_msg);
        } else if let Some(message) = task_message {
            // Send task message to agent
            self.send_message(message).await?;
        }

        Ok(())
    }

    /// Regression: if the agent call for the current plan task errored
    /// (provider/network failure), the event loop used to route it to
    /// `show_error`, which resets `is_processing`/`streaming_response` but
    /// never touched `executing_plan` or the task's status. The task was
    /// left permanently `InProgress`, `executing_plan` stayed `true`, and
    /// no further task was ever dispatched - the plan was stuck with no
    /// recovery path. Mark the in-progress task `Failed` and stop
    /// auto-execution so the user sees what happened and can retry/reject
    /// the plan instead of watching a silently frozen spinner.
    async fn fail_current_plan_task(&mut self, error: &str) -> Result<()> {
        self.executing_plan = false;

        let Some(plan) = &mut self.current_plan else {
            return Ok(());
        };

        if let Some(task) = plan
            .tasks
            .iter_mut()
            .find(|t| matches!(t.status, crate::plan::TaskStatus::InProgress))
        {
            task.status = crate::plan::TaskStatus::Failed;
            task.notes = Some(format!("Task failed: {}", error));
            tracing::warn!(
                "Plan task '{}' failed and auto-execution stopped: {}",
                task.title,
                error
            );
        }

        self.save_plan().await
    }

    /// Show an error message
    fn show_error(&mut self, error: String) {
        self.is_processing = false;
        self.streaming_response = None;
        self.processing_session = None;
        self.error_message = Some(error);
        // Auto-scroll to show the error
        self.scroll_offset = 0;
    }

    /// Switch to a different mode
    async fn switch_mode(&mut self, mode: AppMode) -> Result<()> {
        tracing::info!("🔄 Switching mode to: {:?}", mode);
        self.mode = mode;

        if mode == AppMode::Sessions {
            self.load_sessions().await?;
        }

        Ok(())
    }

    /// Get total token count for current session
    pub fn total_tokens(&self) -> i32 {
        self.messages.iter().filter_map(|m| m.token_count).sum()
    }

    /// Get total cost for current session
    pub fn total_cost(&self) -> f64 {
        self.messages.iter().filter_map(|m| m.cost).sum()
    }

    /// Handle tool approval request
    fn handle_approval_requested(&mut self, request: ToolApprovalRequest) {
        self.pending_approval = Some(request);
        self.show_approval_details = false;
        self.mode = AppMode::ToolApproval;
    }

    /// Handle keys in approval mode
    async fn handle_approval_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;

        if let Some(ref approval_request) = self.pending_approval {
            if keys::is_approve(&event) {
                // User approved
                let response = ToolApprovalResponse {
                    request_id: approval_request.request_id,
                    approved: true,
                    reason: None,
                };

                // Send response back through the channel
                let _ = approval_request.response_tx.send(response.clone());

                // Send event to update UI
                let _ = self
                    .event_sender()
                    .send(TuiEvent::ToolApprovalResponse(response));
            } else if keys::is_deny(&event) || keys::is_cancel(&event) {
                // User denied
                let response = ToolApprovalResponse {
                    request_id: approval_request.request_id,
                    approved: false,
                    reason: Some("User denied permission".to_string()),
                };

                // Send response back through the channel
                let _ = approval_request.response_tx.send(response.clone());

                // Send event to update UI
                let _ = self
                    .event_sender()
                    .send(TuiEvent::ToolApprovalResponse(response));
            } else if keys::is_view_details(&event) {
                // Toggle details view
                self.show_approval_details = !self.show_approval_details;
            }
        }

        Ok(())
    }

    /// Open file picker and populate file list
    async fn open_file_picker(&mut self) -> Result<()> {
        // Get list of files in current directory
        let mut files = Vec::new();

        // Add parent directory option if not at root
        if self.file_picker_current_dir.parent().is_some() {
            files.push(self.file_picker_current_dir.join(".."));
        }

        // Read directory entries
        if let Ok(entries) = std::fs::read_dir(&self.file_picker_current_dir) {
            for entry in entries.flatten() {
                files.push(entry.path());
            }
        }

        // Sort: directories first, then files, alphabetically
        files.sort_by(|a, b| {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        self.file_picker_files = files;
        self.file_picker_selected = 0;
        self.file_picker_scroll_offset = 0;
        self.switch_mode(AppMode::FilePicker).await?;

        Ok(())
    }

    /// Handle keys in file picker mode
    async fn handle_file_picker_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;
        use crossterm::event::KeyCode;

        if keys::is_cancel(&event) {
            // Cancel file picker and return to chat
            self.switch_mode(AppMode::Chat).await?;
        } else if keys::is_up(&event) {
            // Move selection up
            self.file_picker_selected = self.file_picker_selected.saturating_sub(1);

            // Adjust scroll offset if needed
            if self.file_picker_selected < self.file_picker_scroll_offset {
                self.file_picker_scroll_offset = self.file_picker_selected;
            }
        } else if keys::is_down(&event) {
            // Move selection down
            if self.file_picker_selected + 1 < self.file_picker_files.len() {
                self.file_picker_selected += 1;

                // Adjust scroll offset if needed (assuming 20 visible items)
                let visible_items = 20;
                if self.file_picker_selected >= self.file_picker_scroll_offset + visible_items {
                    self.file_picker_scroll_offset = self.file_picker_selected - visible_items + 1;
                }
            }
        } else if keys::is_enter(&event) || event.code == KeyCode::Char(' ') {
            // Select file or navigate into directory
            if let Some(selected_path) = self.file_picker_files.get(self.file_picker_selected) {
                if selected_path.is_dir() {
                    // Navigate into directory
                    if selected_path.ends_with("..") {
                        // Go to parent directory
                        if let Some(parent) = self.file_picker_current_dir.parent() {
                            self.file_picker_current_dir = parent.to_path_buf();
                        }
                    } else {
                        self.file_picker_current_dir = selected_path.clone();
                    }
                    // Refresh file list
                    self.open_file_picker().await?;
                } else {
                    // Insert file path into input buffer, at the cursor.
                    let path_str = selected_path.to_string_lossy().to_string();
                    self.textarea.insert_str(&path_str);
                    self.switch_mode(AppMode::Chat).await?;
                }
            }
        }

        Ok(())
    }

    /// Open the Model Download dialog (Ctrl+D). Shows curated suggestions
    /// immediately; the locally-installed list refreshes in the background
    /// (network call to Ollama) and merges in once it arrives.
    async fn open_model_download(&mut self) -> Result<()> {
        self.model_download_input.clear();
        self.model_download_selected = 0;
        self.model_download_running = false;
        self.model_download_status = None;
        self.model_download_fraction = None;
        self.model_download_confirm_delete = None;
        self.model_download_deleting = None;
        self.refresh_model_download_suggestions();

        let host = self.ollama_host.clone();
        let sender = self.event_sender();
        tokio::spawn(async move {
            let models = super::ollama_download::fetch_installed_models(host).await;
            let _ = sender.send(TuiEvent::OllamaModelsListed(models));
        });

        self.switch_mode(AppMode::ModelDownload).await
    }

    /// Recompute `model_download_suggestions` from the current input text.
    fn refresh_model_download_suggestions(&mut self) {
        self.model_download_suggestions = super::ollama_download::filter_suggestions(
            &self.model_download_input,
            &self.model_download_installed,
        );
        self.model_download_selected = 0;
    }

    /// Start pulling `model` in the background. No-op if a pull is already
    /// running (only one at a time from this dialog).
    async fn start_model_pull(&mut self, model: String) {
        if self.model_download_running || model.trim().is_empty() {
            return;
        }

        self.model_download_running = true;
        self.model_download_status = Some("starting…".to_string());
        self.model_download_fraction = None;

        let host = self.ollama_host.clone();
        let sender = self.event_sender();
        let handle = super::ollama_download::spawn_pull(host, model, sender).await;
        self.model_download_task = Some(handle);
    }

    /// Start deleting `model` in the background. No-op if a pull or delete
    /// is already running (only one operation at a time from this dialog).
    async fn start_model_delete(&mut self, model: String) {
        if self.model_download_running || self.model_download_deleting.is_some() {
            return;
        }

        self.model_download_deleting = Some(model.clone());

        let host = self.ollama_host.clone();
        let sender = self.event_sender();
        let handle = super::ollama_download::spawn_delete(host, model, sender).await;
        self.model_download_delete_task = Some(handle);
    }

    /// Handle keys in the Model Download dialog.
    async fn handle_model_download_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;
        use crossterm::event::KeyCode;

        // Confirming a delete: only Y/Enter confirms, N/Esc cancels back to
        // the suggestion list (without closing the whole dialog).
        if let Some(model) = self.model_download_confirm_delete.clone() {
            match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    self.model_download_confirm_delete = None;
                    self.start_model_delete(model).await;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.model_download_confirm_delete = None;
                }
                _ => {}
            }
            return Ok(());
        }

        if keys::is_cancel(&event) {
            // Cancel an in-flight pull/delete (if any) and close the dialog.
            if let Some(handle) = self.model_download_task.take() {
                handle.abort();
            }
            if let Some(handle) = self.model_download_delete_task.take() {
                handle.abort();
            }
            self.model_download_running = false;
            self.model_download_deleting = None;
            self.model_download_status = None;
            self.model_download_fraction = None;
            self.switch_mode(AppMode::Chat).await?;
            return Ok(());
        }

        // While a pull or delete is running, only Esc (handled above) does anything.
        if self.model_download_running || self.model_download_deleting.is_some() {
            return Ok(());
        }

        if keys::is_up(&event) {
            self.model_download_selected = self.model_download_selected.saturating_sub(1);
        } else if keys::is_down(&event) {
            if !self.model_download_suggestions.is_empty() {
                self.model_download_selected = (self.model_download_selected + 1)
                    .min(self.model_download_suggestions.len() - 1);
            }
        } else if event.code == KeyCode::Tab {
            // Copy the highlighted suggestion into the input for editing/confirmation.
            if let Some(suggestion) = self
                .model_download_suggestions
                .get(self.model_download_selected)
            {
                self.model_download_input = suggestion.clone();
                self.refresh_model_download_suggestions();
            }
        } else if event.code == KeyCode::Delete {
            // Ask for confirmation before deleting the highlighted model -
            // only installed models can be deleted.
            if let Some(name) = self
                .model_download_suggestions
                .get(self.model_download_selected)
            {
                if self.model_download_installed.iter().any(|m| m == name) {
                    self.model_download_confirm_delete = Some(name.clone());
                }
            }
        } else if keys::is_enter(&event) {
            let model = self.model_download_input.trim().to_string();
            if !model.is_empty() {
                self.start_model_pull(model).await;
            }
        } else {
            match event.code {
                KeyCode::Char(c) => {
                    self.model_download_input.push(c);
                    self.refresh_model_download_suggestions();
                }
                KeyCode::Backspace => {
                    self.model_download_input.pop();
                    self.refresh_model_download_suggestions();
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Open the Provider Switch dialog (Ctrl+W). Fetches the list of
    /// locally-installed Ollama models in the background; the dialog opens
    /// immediately showing a loading state until they arrive.
    async fn open_provider_switch(&mut self) -> Result<()> {
        self.provider_switch_selected = 0;
        self.provider_switch_models.clear();
        self.provider_switch_loading = true;

        let host = self.ollama_host.clone();
        let sender = self.event_sender();
        tokio::spawn(async move {
            let models = super::ollama_download::fetch_installed_models(host).await;
            let _ = sender.send(TuiEvent::ProviderSwitchModelsListed(models));
        });

        self.switch_mode(AppMode::ProviderSwitch).await
    }

    /// Handle keys in the Provider Switch dialog.
    async fn handle_provider_switch_key(
        &mut self,
        event: crossterm::event::KeyEvent,
    ) -> Result<()> {
        use super::events::keys;

        if keys::is_cancel(&event) {
            self.switch_mode(AppMode::Chat).await?;
            return Ok(());
        }

        // While the model list is still loading, only Esc (handled above)
        // does anything.
        if self.provider_switch_loading {
            return Ok(());
        }

        if keys::is_up(&event) {
            self.provider_switch_selected = self.provider_switch_selected.saturating_sub(1);
        } else if keys::is_down(&event) {
            if !self.provider_switch_models.is_empty() {
                self.provider_switch_selected =
                    (self.provider_switch_selected + 1).min(self.provider_switch_models.len() - 1);
            }
        } else if keys::is_enter(&event) {
            if let Some(model) = self
                .provider_switch_models
                .get(self.provider_switch_selected)
                .cloned()
            {
                self.switch_provider_to_ollama_model(model).await?;
            }
        }

        Ok(())
    }

    /// Switch the active provider to the native Ollama provider running
    /// `model` on `self.ollama_host`, in place - preserving the tool
    /// registry, approval callback, and every other `AgentService` setting
    /// (see `AgentService::set_provider`, which mutates the provider field
    /// only rather than rebuilding the service from scratch).
    ///
    /// Fails safely with a visible error instead of switching if a request
    /// is currently in flight: a background task may be holding a clone of
    /// `agent_service` at that moment, in which case `Arc::get_mut` simply
    /// returns `None` rather than allowing an unsafe in-place mutation.
    async fn switch_provider_to_ollama_model(&mut self, model: String) -> Result<()> {
        match super::ollama_download::build_ollama_provider(
            &self.ollama_host,
            &model,
            self.ollama_config.as_ref(),
        ) {
            Ok(provider) => match Arc::get_mut(&mut self.agent_service) {
                Some(service) => {
                    service.set_provider(provider);

                    // Re-stamp the session: the header and every message bubble
                    // render the model from `session.model`, so without this the
                    // UI keeps showing the *old* model after a successful switch.
                    if let Some(session) = &mut self.current_session {
                        session.model = Some(model.clone());
                        session.provider = Some("ollama".to_string());
                        if let Err(e) = self.session_service.update_session(session).await {
                            tracing::warn!("Failed to update session after model switch: {}", e);
                        }
                    }

                    let notification = DisplayMessage {
                        id: Uuid::new_v4(),
                        role: "system".to_string(),
                        content: format!("✅ Switched to Ollama model '{model}'."),
                        thinking_text: None,
                        thinking_expanded: false,
                        timestamp: chrono::Utc::now(),
                        token_count: None,
                        cost: None,
                        provider_name: None,
                        perf_metrics: None,
                        tokens_per_second: None,
                    };
                    self.messages.push(notification);
                    self.switch_mode(AppMode::Chat).await?;
                }
                None => {
                    self.error_message = Some(
                        "Can't switch provider while a response is in progress - try again once it finishes."
                            .to_string(),
                    );
                }
            },
            Err(e) => {
                self.error_message = Some(e);
            }
        }

        Ok(())
    }

    /// Open the llama.cpp Local Models dialog (Ctrl+G). Scans
    /// `llama_cpp_models_dir` in the background; the dialog opens
    /// immediately showing a loading state until the scan completes.
    async fn open_llama_cpp_models(&mut self) -> Result<()> {
        self.llama_cpp_download_input.clear();
        self.llama_cpp_selected = 0;
        self.llama_cpp_download_running = false;
        self.llama_cpp_download_status = None;
        self.llama_cpp_download_fraction = None;
        self.llama_cpp_confirm_delete = None;
        self.llama_cpp_deleting = None;
        self.llama_cpp_loading = true;

        let models_dir = self.llama_cpp_models_dir.clone();
        let extra_model_paths = self.llama_cpp_extra_model_paths.clone();
        let ollama_models_dir = self.llama_cpp_ollama_models_dir.clone();
        let sender = self.event_sender();
        tokio::spawn(async move {
            let models = super::llama_cpp_download::list_local(
                models_dir,
                extra_model_paths,
                ollama_models_dir,
            )
            .await;
            let _ = sender.send(TuiEvent::LlamaCppModelsListed(models));
        });

        self.switch_mode(AppMode::LlamaCppModelPicker).await
    }

    /// Start downloading the current input text as a new `.gguf` source. No-op
    /// if a download is already running or the input is empty.
    async fn start_llama_cpp_download(&mut self) {
        let source = self.llama_cpp_download_input.trim().to_string();
        if self.llama_cpp_download_running || source.is_empty() {
            return;
        }

        self.llama_cpp_download_running = true;
        self.llama_cpp_download_status = Some("resolving…".to_string());
        self.llama_cpp_download_fraction = None;

        let models_dir = self.llama_cpp_models_dir.clone();
        let sender = self.event_sender();
        let handle = super::llama_cpp_download::spawn_download(source, models_dir, sender).await;
        self.llama_cpp_download_task = Some(handle);
    }

    /// Start deleting `path` in the background. No-op if a download,
    /// delete, or switch is already in flight.
    async fn start_llama_cpp_delete(&mut self, path: std::path::PathBuf) {
        if self.llama_cpp_download_running
            || self.llama_cpp_deleting.is_some()
            || self.llama_cpp_switching.is_some()
        {
            return;
        }

        self.llama_cpp_deleting = Some(path.clone());

        let sender = self.event_sender();
        let handle = super::llama_cpp_download::spawn_delete(path, sender).await;
        self.llama_cpp_delete_task = Some(handle);
    }

    /// Start switching the active provider to `path` in the background. This
    /// blocks (via `spawn_blocking`) while the model loads, so the dialog
    /// shows a "Loading model…" state rather than switching instantly the
    /// way Ollama's Ctrl+W does - see `llama_cpp_download`'s module doc.
    async fn start_llama_cpp_switch(&mut self, path: std::path::PathBuf) {
        if self.llama_cpp_download_running
            || self.llama_cpp_deleting.is_some()
            || self.llama_cpp_switching.is_some()
        {
            return;
        }

        self.llama_cpp_switching = Some(path.clone());

        let config = self.llama_cpp_config.clone();
        let slot = self.llama_cpp_pending_provider.clone();
        let sender = self.event_sender();
        let handle = super::llama_cpp_download::spawn_switch(path, config, slot, sender).await;
        self.llama_cpp_switch_task = Some(handle);
    }

    /// Handle keys in the llama.cpp Local Models dialog.
    async fn handle_llama_cpp_models_key(
        &mut self,
        event: crossterm::event::KeyEvent,
    ) -> Result<()> {
        use super::events::keys;
        use crossterm::event::KeyCode;

        // Confirming a delete: only Y/Enter confirms, N/Esc cancels back to
        // the list (without closing the whole dialog).
        if let Some(path) = self.llama_cpp_confirm_delete.clone() {
            match event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    self.llama_cpp_confirm_delete = None;
                    self.start_llama_cpp_delete(path).await;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.llama_cpp_confirm_delete = None;
                }
                _ => {}
            }
            return Ok(());
        }

        if keys::is_cancel(&event) {
            // Cancel an in-flight download/delete/switch (if any) and close
            // the dialog. A switch that already swapped the provider (event
            // already processed) can't be "cancelled" here - only an
            // in-flight load can, since `llama_cpp_switching` is cleared the
            // moment `LlamaCppSwitchFinished` lands.
            if let Some(handle) = self.llama_cpp_download_task.take() {
                handle.abort();
            }
            if let Some(handle) = self.llama_cpp_delete_task.take() {
                handle.abort();
            }
            if let Some(handle) = self.llama_cpp_switch_task.take() {
                handle.abort();
            }
            self.llama_cpp_download_running = false;
            self.llama_cpp_deleting = None;
            self.llama_cpp_switching = None;
            self.llama_cpp_download_status = None;
            self.llama_cpp_download_fraction = None;
            self.switch_mode(AppMode::Chat).await?;
            return Ok(());
        }

        // While a download/delete/switch is running, only Esc (handled
        // above) does anything.
        if self.llama_cpp_download_running
            || self.llama_cpp_deleting.is_some()
            || self.llama_cpp_switching.is_some()
        {
            return Ok(());
        }

        if keys::is_up(&event) {
            self.llama_cpp_selected = self.llama_cpp_selected.saturating_sub(1);
        } else if keys::is_down(&event) {
            if !self.llama_cpp_models.is_empty() {
                self.llama_cpp_selected =
                    (self.llama_cpp_selected + 1).min(self.llama_cpp_models.len() - 1);
            }
        } else if event.code == KeyCode::Delete {
            if let Some(model) = self.llama_cpp_models.get(self.llama_cpp_selected) {
                self.llama_cpp_confirm_delete = Some(model.path.clone());
            }
        } else if keys::is_enter(&event) {
            if !self.llama_cpp_download_input.trim().is_empty() {
                // Typed text takes priority: download it as a new source.
                self.start_llama_cpp_download().await;
            } else if let Some(model) = self.llama_cpp_models.get(self.llama_cpp_selected).cloned()
            {
                // Empty input: switch to the highlighted local model.
                self.start_llama_cpp_switch(model.path).await;
            }
        } else {
            match event.code {
                KeyCode::Char(c) => {
                    self.llama_cpp_download_input.push(c);
                }
                KeyCode::Backspace => {
                    self.llama_cpp_download_input.pop();
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_message_from_db_message() {
        let msg = Message {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Hello".to_string(),
            sequence: 1,
            created_at: chrono::Utc::now(),
            token_count: Some(10),
            cost: Some(0.001),
            provider_name: None,
            perf_metrics_json: None,
        };

        let display_msg: DisplayMessage = msg.into();
        assert_eq!(display_msg.role, "user");
        assert_eq!(display_msg.content, "Hello");
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_context_and_chat_template_for_path_reads_a_real_header() {
        // Hand-crafted minimal GGUF bytes, same pattern as
        // gguf_metadata.rs's own tests - magic + version + zero tensors +
        // two KV pairs (a context_length key and chat_template presence).
        fn push_string(buf: &mut Vec<u8>, s: &str) {
            buf.extend_from_slice(&(s.len() as u64).to_le_bytes());
            buf.extend_from_slice(s.as_bytes());
        }
        let mut buf = Vec::new();
        buf.extend_from_slice(b"GGUF");
        buf.extend_from_slice(&3u32.to_le_bytes()); // version
        buf.extend_from_slice(&0u64.to_le_bytes()); // tensor_count
        buf.extend_from_slice(&2u64.to_le_bytes()); // kv_count

        push_string(&mut buf, "qwen2.context_length");
        buf.extend_from_slice(&4u32.to_le_bytes()); // ValueType::U32
        buf.extend_from_slice(&32768u32.to_le_bytes());

        push_string(&mut buf, "tokenizer.chat_template");
        buf.extend_from_slice(&8u32.to_le_bytes()); // ValueType::String
        push_string(&mut buf, "{{ messages }}");

        let file = tempfile::NamedTempFile::new().expect("temp file");
        std::fs::write(file.path(), &buf).expect("write fixture");

        let (context_length, has_chat_template) =
            llama_cpp_context_and_chat_template_for_path(file.path());
        assert_eq!(context_length, Some(32768));
        assert!(has_chat_template);
    }

    #[cfg(feature = "gguf-management")]
    #[test]
    fn llama_cpp_context_and_chat_template_for_path_degrades_cleanly_on_a_missing_file() {
        let path = std::path::Path::new("/definitely/does/not/exist/crustly-test.gguf");
        assert_eq!(
            llama_cpp_context_and_chat_template_for_path(path),
            (None, false)
        );
    }

    use crate::db::Database;
    use crate::llm::provider::{
        LLMRequest, LLMResponse, Provider, ProviderStream, Result as ProviderResult,
    };
    use crossterm::event::{KeyCode, KeyModifiers};

    /// Minimal `Provider` stub - these tests only exercise the Model
    /// Download dialog's state machine, never an actual chat request.
    struct DummyProvider;

    #[async_trait::async_trait]
    impl Provider for DummyProvider {
        async fn complete(&self, _request: LLMRequest) -> ProviderResult<LLMResponse> {
            unimplemented!("dialog tests never call complete()")
        }
        async fn stream(&self, _request: LLMRequest) -> ProviderResult<ProviderStream> {
            unimplemented!("dialog tests never call stream()")
        }
        fn name(&self) -> &str {
            "dummy"
        }
        fn default_model(&self) -> &str {
            "dummy-model"
        }
        fn supported_models(&self) -> Vec<String> {
            vec!["dummy-model".to_string()]
        }
        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(4096)
        }
        fn calculate_cost(&self, _model: &str, _input: u32, _output: u32) -> f64 {
            0.0
        }
    }

    async fn test_app() -> App {
        let db = Database::connect_in_memory().await.expect("in-memory db");
        db.run_migrations().await.expect("run migrations");
        let context = ServiceContext::new(db.pool().clone());
        let agent_service = Arc::new(AgentService::new(Arc::new(DummyProvider), context.clone()));
        App::new(agent_service, context)
    }

    fn key(code: KeyCode) -> crossterm::event::KeyEvent {
        crossterm::event::KeyEvent::new(code, KeyModifiers::empty())
    }

    #[tokio::test]
    async fn open_model_download_switches_mode_and_seeds_suggestions() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();

        assert_eq!(app.mode, AppMode::ModelDownload);
        assert!(app.model_download_input.is_empty());
        assert!(
            !app.model_download_suggestions.is_empty(),
            "curated models should seed the suggestion list immediately"
        );
    }

    #[tokio::test]
    async fn model_download_typing_filters_suggestions() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();

        app.handle_model_download_key(key(KeyCode::Char('l')))
            .await
            .unwrap();
        app.handle_model_download_key(key(KeyCode::Char('l')))
            .await
            .unwrap();

        assert_eq!(app.model_download_input, "ll");
        assert!(app
            .model_download_suggestions
            .iter()
            .all(|s| s.to_lowercase().contains("ll")));
    }

    #[tokio::test]
    async fn model_download_backspace_removes_last_char() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();
        app.handle_model_download_key(key(KeyCode::Char('a')))
            .await
            .unwrap();
        app.handle_model_download_key(key(KeyCode::Backspace))
            .await
            .unwrap();
        assert!(app.model_download_input.is_empty());
    }

    #[tokio::test]
    async fn model_download_tab_adopts_highlighted_suggestion() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();
        let first_suggestion = app.model_download_suggestions[0].clone();

        app.handle_model_download_key(key(KeyCode::Tab))
            .await
            .unwrap();

        assert_eq!(app.model_download_input, first_suggestion);
    }

    #[tokio::test]
    async fn model_download_esc_closes_dialog_without_running_pull() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();

        app.handle_model_download_key(key(KeyCode::Esc))
            .await
            .unwrap();

        assert_eq!(app.mode, AppMode::Chat);
        assert!(!app.model_download_running);
    }

    #[tokio::test]
    async fn model_download_enter_starts_pull_then_esc_aborts_it() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();
        app.model_download_input = "qwen2.5-coder:7b".to_string();

        app.handle_model_download_key(key(KeyCode::Enter))
            .await
            .unwrap();
        assert!(app.model_download_running);

        // Esc while a pull is running must abort it and return to chat,
        // rather than just closing the dialog.
        app.handle_model_download_key(key(KeyCode::Esc))
            .await
            .unwrap();
        assert!(!app.model_download_running);
        assert_eq!(app.mode, AppMode::Chat);
    }

    #[tokio::test]
    async fn handle_ollama_models_listed_updates_installed_list() {
        let mut app = test_app().await;
        app.handle_event(TuiEvent::OllamaModelsListed(vec![
            "custom-model:1b".to_string()
        ]))
        .await
        .unwrap();

        assert_eq!(
            app.model_download_installed,
            vec!["custom-model:1b".to_string()]
        );
    }

    #[tokio::test]
    async fn handle_ollama_pull_progress_updates_status_and_fraction() {
        let mut app = test_app().await;
        app.handle_event(TuiEvent::OllamaPullProgress(
            super::super::ollama_download::ModelPullProgress {
                status: "pulling abc123".to_string(),
                total: Some(100),
                completed: Some(50),
            },
        ))
        .await
        .unwrap();

        assert_eq!(
            app.model_download_status,
            Some("pulling abc123".to_string())
        );
        assert_eq!(app.model_download_fraction, Some(0.5));
    }

    #[tokio::test]
    async fn handle_ollama_pull_finished_success_posts_chat_message() {
        let mut app = test_app().await;
        app.mode = AppMode::ModelDownload;
        app.model_download_running = true;

        app.handle_event(TuiEvent::OllamaPullFinished {
            model: "qwen2.5-coder:7b".to_string(),
            error: None,
        })
        .await
        .unwrap();

        assert!(!app.model_download_running);
        assert_eq!(app.mode, AppMode::Chat);
        assert!(app
            .messages
            .iter()
            .any(|m| m.content.contains("Pulled 'qwen2.5-coder:7b'")));
    }

    #[tokio::test]
    async fn handle_ollama_pull_finished_failure_posts_error_message() {
        let mut app = test_app().await;

        app.handle_event(TuiEvent::OllamaPullFinished {
            model: "bogus-model".to_string(),
            error: Some("model not found".to_string()),
        })
        .await
        .unwrap();

        assert!(app
            .messages
            .iter()
            .any(|m| m.content.contains("Failed to pull 'bogus-model'")
                && m.content.contains("model not found")));
    }

    /// Delete only asks for confirmation when the highlighted suggestion is
    /// actually installed - curated-but-not-pulled entries can't be deleted.
    #[tokio::test]
    async fn delete_key_ignored_for_uninstalled_suggestion() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();
        // Freshly opened dialog has no installed models yet.
        assert!(app.model_download_installed.is_empty());

        app.handle_model_download_key(key(KeyCode::Delete))
            .await
            .unwrap();

        assert!(app.model_download_confirm_delete.is_none());
    }

    #[tokio::test]
    async fn delete_key_on_installed_model_asks_for_confirmation() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();
        app.handle_event(TuiEvent::OllamaModelsListed(vec![
            "qwen2.5-coder:7b".to_string()
        ]))
        .await
        .unwrap();
        let idx = app
            .model_download_suggestions
            .iter()
            .position(|s| s == "qwen2.5-coder:7b")
            .expect("installed model should be in suggestions");
        app.model_download_selected = idx;

        app.handle_model_download_key(key(KeyCode::Delete))
            .await
            .unwrap();

        assert_eq!(
            app.model_download_confirm_delete,
            Some("qwen2.5-coder:7b".to_string())
        );
    }

    #[tokio::test]
    async fn confirm_delete_n_cancels_back_to_list() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();
        app.model_download_confirm_delete = Some("qwen2.5-coder:7b".to_string());

        app.handle_model_download_key(key(KeyCode::Char('n')))
            .await
            .unwrap();

        assert!(app.model_download_confirm_delete.is_none());
        assert!(app.model_download_deleting.is_none());
        assert_eq!(app.mode, AppMode::ModelDownload);
    }

    #[tokio::test]
    async fn confirm_delete_esc_cancels_back_to_list_without_closing_dialog() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();
        app.model_download_confirm_delete = Some("qwen2.5-coder:7b".to_string());

        app.handle_model_download_key(key(KeyCode::Esc))
            .await
            .unwrap();

        assert!(app.model_download_confirm_delete.is_none());
        assert_eq!(
            app.mode,
            AppMode::ModelDownload,
            "Esc during confirmation should not close the whole dialog"
        );
    }

    #[tokio::test]
    async fn confirm_delete_y_starts_delete() {
        let mut app = test_app().await;
        app.open_model_download().await.unwrap();
        app.model_download_confirm_delete = Some("qwen2.5-coder:7b".to_string());

        app.handle_model_download_key(key(KeyCode::Char('y')))
            .await
            .unwrap();

        assert!(app.model_download_confirm_delete.is_none());
        assert_eq!(
            app.model_download_deleting,
            Some("qwen2.5-coder:7b".to_string())
        );
    }

    #[tokio::test]
    async fn handle_ollama_delete_finished_success_removes_from_installed_and_posts_message() {
        let mut app = test_app().await;
        app.mode = AppMode::ModelDownload;
        app.model_download_installed = vec!["qwen2.5-coder:7b".to_string()];
        app.model_download_deleting = Some("qwen2.5-coder:7b".to_string());

        app.handle_event(TuiEvent::OllamaDeleteFinished {
            model: "qwen2.5-coder:7b".to_string(),
            error: None,
        })
        .await
        .unwrap();

        assert!(app.model_download_deleting.is_none());
        assert_eq!(app.mode, AppMode::Chat);
        assert!(!app
            .model_download_installed
            .contains(&"qwen2.5-coder:7b".to_string()));
        assert!(app
            .messages
            .iter()
            .any(|m| m.content.contains("Deleted 'qwen2.5-coder:7b'")));
    }

    #[tokio::test]
    async fn handle_ollama_delete_finished_failure_keeps_installed_and_posts_error() {
        let mut app = test_app().await;
        app.model_download_installed = vec!["qwen2.5-coder:7b".to_string()];
        app.model_download_deleting = Some("qwen2.5-coder:7b".to_string());

        app.handle_event(TuiEvent::OllamaDeleteFinished {
            model: "qwen2.5-coder:7b".to_string(),
            error: Some("model is in use".to_string()),
        })
        .await
        .unwrap();

        assert!(app.model_download_deleting.is_none());
        assert!(app
            .model_download_installed
            .contains(&"qwen2.5-coder:7b".to_string()));
        assert!(app.messages.iter().any(|m| m
            .content
            .contains("Failed to delete 'qwen2.5-coder:7b'")
            && m.content.contains("model is in use")));
    }

    /// Up recalls the last submitted message into the input without resending
    /// it, and Down walks back toward the newest, then restores the draft.
    #[tokio::test]
    async fn up_recalls_previous_messages_without_sending_them() {
        let mut app = test_app().await;
        app.push_input_history("first message");
        app.push_input_history("second message");

        // Typing a draft, then Up: the draft is stashed, newest entry loaded.
        app.set_input_text("half-typed draft");
        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.input_text(), "second message");

        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.input_text(), "first message");

        // Oldest entry: further Up stays put rather than wrapping around.
        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.input_text(), "first message");

        // Down walks forward, then restores the draft that was interrupted.
        app.handle_chat_key(key(KeyCode::Down)).await.unwrap();
        assert_eq!(app.input_text(), "second message");
        app.handle_chat_key(key(KeyCode::Down)).await.unwrap();
        assert_eq!(app.input_text(), "half-typed draft");

        // Nothing was sent: recall only fills the input.
        assert!(
            app.messages.is_empty(),
            "recalling history must not send anything"
        );
    }

    /// Regression: the Kitty keyboard protocol (enabled for Shift+Enter) makes
    /// crossterm emit Release as well as Press. Handling both ran every handler
    /// twice per physical keypress - Up recalled an entry on Press and stepped
    /// straight past it on Release, so history recall did nothing at all in the
    /// real TUI while passing tests that synthesise a bare Press.
    #[tokio::test]
    async fn key_release_events_are_ignored() {
        use crossterm::event::{KeyEvent, KeyEventKind};

        let mut app = test_app().await;
        // handle_event dispatches by mode (the app starts in Splash), unlike the
        // other tests, which call handle_chat_key directly.
        app.mode = AppMode::Chat;
        app.push_input_history("first message");
        app.push_input_history("second message");

        // One physical press of Up: crossterm delivers Press then Release.
        app.handle_event(TuiEvent::Key(KeyEvent::new(
            KeyCode::Up,
            KeyModifiers::NONE,
        )))
        .await
        .unwrap();

        let mut release = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        release.kind = KeyEventKind::Release;
        app.handle_event(TuiEvent::Key(release)).await.unwrap();

        assert_eq!(
            app.input_text(),
            "second message",
            "one keypress must recall exactly one entry; the Release event \
             double-stepped it back to 'first message'"
        );
    }

    /// The recalled text must be editable and resendable - the whole point of
    /// the feature - and the cursor must sit at the end, ready to type.
    #[tokio::test]
    async fn recalled_message_can_be_edited_before_resending() {
        let mut app = test_app().await;
        app.push_input_history("list files");

        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.input_text(), "list files");

        // Cursor lands at the end, so typing appends rather than prepends.
        app.handle_chat_key(key(KeyCode::Char('!'))).await.unwrap();
        assert_eq!(app.input_text(), "list files!");
    }

    /// Up/Down must still move the cursor inside a multi-line draft (Shift+Enter
    /// makes those), so history is only recalled from the first/last line.
    #[tokio::test]
    async fn up_moves_the_cursor_inside_a_multiline_draft() {
        let mut app = test_app().await;
        app.push_input_history("old message");

        app.set_input_text("line one\nline two");
        // Cursor is on the last line, so Up moves it up rather than recalling.
        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(
            app.input_text(),
            "line one\nline two",
            "Up on a lower line must move the cursor, not overwrite the draft"
        );
        assert_eq!(app.textarea.cursor().0, 0, "cursor should have moved up");

        // Now on the first line, Up does recall.
        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.input_text(), "old message");
    }

    /// With no history, Up/Down must behave exactly as before.
    #[tokio::test]
    async fn up_is_plain_cursor_movement_when_there_is_no_history() {
        let mut app = test_app().await;
        app.set_input_text("just typing");

        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.input_text(), "just typing");
        app.handle_chat_key(key(KeyCode::Down)).await.unwrap();
        assert_eq!(app.input_text(), "just typing");
    }

    /// A shell does not make you press Up twice to get past a repeated command.
    #[tokio::test]
    async fn consecutive_duplicate_submissions_are_stored_once() {
        let mut app = test_app().await;
        app.push_input_history("same");
        app.push_input_history("same");
        app.push_input_history("different");

        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.input_text(), "different");
        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.input_text(), "same");
        app.handle_chat_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(
            app.input_text(),
            "same",
            "the duplicate was not stored twice"
        );
    }

    fn key_mod(code: KeyCode, modifiers: KeyModifiers) -> crossterm::event::KeyEvent {
        crossterm::event::KeyEvent::new(code, modifiers)
    }

    #[tokio::test]
    async fn chat_shift_enter_inserts_newline_instead_of_submitting() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.handle_chat_key(key(KeyCode::Char('h'))).await.unwrap();
        app.handle_chat_key(key(KeyCode::Char('i'))).await.unwrap();
        app.handle_chat_key(key_mod(KeyCode::Enter, KeyModifiers::SHIFT))
            .await
            .unwrap();
        app.handle_chat_key(key(KeyCode::Char('!'))).await.unwrap();

        assert_eq!(app.input_text(), "hi\n!");
        assert_eq!(app.mode, AppMode::Chat, "Shift+Enter must not submit");
    }

    #[tokio::test]
    async fn chat_alt_enter_inserts_newline_as_non_kitty_fallback() {
        let mut app = test_app().await;
        app.handle_chat_key(key(KeyCode::Char('x'))).await.unwrap();
        app.handle_chat_key(key_mod(KeyCode::Enter, KeyModifiers::ALT))
            .await
            .unwrap();

        assert_eq!(app.input_text(), "x\n");
    }

    #[tokio::test]
    async fn chat_left_arrow_moves_cursor_for_mid_buffer_insert() {
        let mut app = test_app().await;
        for c in "helo".chars() {
            app.handle_chat_key(key(KeyCode::Char(c))).await.unwrap();
        }
        // Cursor is after "helo"; move back 1 to sit between 'l' and 'o',
        // then insert 'l' - a real cursor means this edits mid-buffer
        // instead of always appending at the end.
        app.handle_chat_key(key(KeyCode::Left)).await.unwrap();
        app.handle_chat_key(key(KeyCode::Char('l'))).await.unwrap();

        assert_eq!(app.input_text(), "hello");
    }

    #[tokio::test]
    async fn chat_backspace_deletes_at_cursor_not_always_the_last_char() {
        let mut app = test_app().await;
        for c in "helllo".chars() {
            app.handle_chat_key(key(KeyCode::Char(c))).await.unwrap();
        }
        // Cursor is after the trailing "o"; move back 2 to sit right after
        // the extra 'l' (index: h-e-l-l-l-o, cursor after 4th char "helll"),
        // then backspace should remove that extra 'l', not the trailing 'o'.
        app.handle_chat_key(key(KeyCode::Left)).await.unwrap();
        app.handle_chat_key(key(KeyCode::Left)).await.unwrap();
        app.handle_chat_key(key(KeyCode::Backspace)).await.unwrap();

        assert_eq!(app.input_text(), "hello");
    }

    #[tokio::test]
    async fn chat_home_and_end_move_cursor_to_line_boundaries() {
        let mut app = test_app().await;
        for c in "hello".chars() {
            app.handle_chat_key(key(KeyCode::Char(c))).await.unwrap();
        }
        assert_eq!(app.textarea.cursor(), (0, 5));

        app.handle_chat_key(key(KeyCode::Home)).await.unwrap();
        assert_eq!(app.textarea.cursor(), (0, 0));

        app.handle_chat_key(key(KeyCode::End)).await.unwrap();
        assert_eq!(app.textarea.cursor(), (0, 5));
    }

    #[tokio::test]
    async fn chat_ctrl_left_right_jump_by_word() {
        let mut app = test_app().await;
        for c in "hello world".chars() {
            app.handle_chat_key(key(KeyCode::Char(c))).await.unwrap();
        }
        assert_eq!(app.textarea.cursor(), (0, 11));

        app.handle_chat_key(key_mod(KeyCode::Left, KeyModifiers::CONTROL))
            .await
            .unwrap();
        // Cursor should now sit at the start of "world", not just move one
        // character back.
        assert_eq!(app.textarea.cursor(), (0, 6));
    }

    #[tokio::test]
    async fn chat_ctrl_backspace_deletes_whole_word() {
        let mut app = test_app().await;
        for c in "hello world".chars() {
            app.handle_chat_key(key(KeyCode::Char(c))).await.unwrap();
        }
        app.handle_chat_key(key_mod(KeyCode::Backspace, KeyModifiers::CONTROL))
            .await
            .unwrap();

        assert_eq!(app.input_text(), "hello ");
    }

    /// Pasting a Windows path must keep its backslashes, and a multi-line paste
    /// must arrive whole. Bracketed paste delivers the text as one block; the
    /// characters are inserted verbatim and nothing is treated as an escape.
    #[tokio::test]
    async fn paste_preserves_backslashes_and_newlines() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        let pasted = "D:\\Projets\\test-crustly\\src\\main.rs\nC:\\Users\\jerem\\.crustly";
        app.handle_event(TuiEvent::Paste(pasted.to_string()))
            .await
            .unwrap();

        assert_eq!(
            app.input_text(),
            pasted,
            "pasted text must survive verbatim - backslashes included"
        );
    }

    /// If the terminal does not support bracketed paste, the text arrives as one
    /// key event per character. Those must land in the input verbatim too - in
    /// particular a backslash must not be swallowed by any shortcut.
    /// AltGr on many non-US layouts produces `\`, and crossterm reports it as
    /// CONTROL|ALT. If the input path drops modified Char events, the backslash
    /// never lands - which is exactly "the backslashes are removed".
    #[tokio::test]
    async fn altgr_backslash_reaches_the_input() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        // A whole Windows path, as AltGr delivers it on a non-US layout.
        for c in r"D:\Projets\test-crustly".chars() {
            let ev = if c == '\\' {
                key_mod(KeyCode::Char(c), KeyModifiers::CONTROL | KeyModifiers::ALT)
            } else {
                key(KeyCode::Char(c))
            };
            app.handle_chat_key(ev).await.unwrap();
        }

        assert_eq!(
            app.input_text(),
            r"D:\Projets\test-crustly",
            "AltGr backslashes must be inserted, not swallowed as control keys"
        );
    }

    /// `@` opens the file picker - but on AZERTY `@` is itself typed with AltGr,
    /// so an AltGr '@' must insert the character rather than open the picker.
    #[tokio::test]
    async fn altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.handle_chat_key(key_mod(
            KeyCode::Char('@'),
            KeyModifiers::CONTROL | KeyModifiers::ALT,
        ))
        .await
        .unwrap();

        assert_eq!(app.input_text(), "@");
        assert_eq!(app.mode, AppMode::Chat, "must not open the file picker");
    }

    /// A plain (unmodified) '@' still opens the file picker.
    #[tokio::test]
    async fn plain_at_sign_still_opens_the_file_picker() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.handle_chat_key(key(KeyCode::Char('@'))).await.unwrap();

        assert_ne!(app.mode, AppMode::Chat, "plain @ should open the picker");
    }

    #[tokio::test]
    async fn typed_backslashes_reach_the_input() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        let path = r"D:\Projets\test-crustly\src";
        for c in path.chars() {
            app.handle_chat_key(key(KeyCode::Char(c))).await.unwrap();
        }

        assert_eq!(app.input_text(), path);
    }

    #[tokio::test]
    async fn paste_inserts_at_cursor_not_always_appended_at_the_end() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        for c in "helo".chars() {
            app.handle_chat_key(key(KeyCode::Char(c))).await.unwrap();
        }
        app.handle_chat_key(key(KeyCode::Left)).await.unwrap();

        app.handle_event(TuiEvent::Paste("l".to_string()))
            .await
            .unwrap();

        assert_eq!(app.input_text(), "hello");
    }

    #[tokio::test]
    async fn paste_with_embedded_newline_produces_multiple_lines() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.handle_event(TuiEvent::Paste("line one\nline two".to_string()))
            .await
            .unwrap();

        assert_eq!(app.input_text(), "line one\nline two");
    }

    #[tokio::test]
    async fn ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.handle_chat_key(key_mod(KeyCode::Char('y'), KeyModifiers::CONTROL))
            .await
            .unwrap();

        assert_eq!(
            app.error_message.as_deref(),
            Some("No response to copy yet.")
        );
    }

    #[tokio::test]
    async fn ctrl_y_copies_last_code_block_when_present() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.messages.push(DisplayMessage {
            id: Uuid::new_v4(),
            role: "assistant".to_string(),
            content: "Here you go:\n\n```rust\nfn main() {}\n```".to_string(),
            thinking_text: None,
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: None,
            cost: None,
            provider_name: None,
            perf_metrics: None,
            tokens_per_second: None,
        });

        app.handle_chat_key(key_mod(KeyCode::Char('y'), KeyModifiers::CONTROL))
            .await
            .unwrap();

        // Clipboard availability is platform/environment-dependent: headless
        // Linux CI has no backend and must fail gracefully with a
        // clipboard-specific error (not the "no response" one); macOS/
        // Windows CI runners typically have a real system clipboard, in
        // which case the extracted code block must actually be there.
        match &app.error_message {
            Some(err) => assert!(err.contains("clipboard"), "unexpected error: {err}"),
            None => {
                let copied = arboard::Clipboard::new()
                    .and_then(|mut cb| cb.get_text())
                    .expect("clipboard copy just succeeded, so read-back must too");
                assert_eq!(copied, "fn main() {}");
            }
        }
    }

    #[tokio::test]
    async fn ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.handle_chat_key(key_mod(KeyCode::Char('v'), KeyModifiers::CONTROL))
            .await
            .unwrap();

        // Clipboard availability is platform/environment-dependent (see
        // ctrl_y_copies_last_code_block_when_present). On a headless Linux
        // CI runner with no backend, this must fail gracefully into
        // error_message rather than panic, and must not insert anything
        // into the input. On macOS/Windows CI, a real clipboard read
        // succeeds (with whatever text happens to be there) and the only
        // requirement is that it doesn't panic.
        if let Some(err) = &app.error_message {
            assert!(err.contains("clipboard"), "unexpected error: {err}");
            assert!(app.textarea.is_empty());
        }
    }

    #[tokio::test]
    async fn auto_mode_defaults_to_interactive() {
        let app = test_app().await;
        assert_eq!(app.auto_mode(), PlanExecMode::Interactive);
    }

    #[tokio::test]
    async fn shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps() {
        let mut app = test_app().await;

        app.handle_key_event(key(KeyCode::BackTab)).await.unwrap();
        assert_eq!(app.auto_mode(), PlanExecMode::AutoPlan);

        app.handle_key_event(key(KeyCode::BackTab)).await.unwrap();
        assert_eq!(app.auto_mode(), PlanExecMode::FullAuto);

        // Wraps back to Interactive - Auto Mode is not a one-way ratchet.
        app.handle_key_event(key(KeyCode::BackTab)).await.unwrap();
        assert_eq!(app.auto_mode(), PlanExecMode::Interactive);
    }

    #[tokio::test]
    async fn shift_tab_works_from_any_mode_not_just_chat() {
        let mut app = test_app().await;
        app.mode = AppMode::Plan;

        app.handle_key_event(key(KeyCode::BackTab)).await.unwrap();

        assert_eq!(app.auto_mode(), PlanExecMode::AutoPlan);
        // Must not have also changed the current screen/dialog mode.
        assert_eq!(app.mode, AppMode::Plan);
    }

    #[tokio::test]
    async fn setting_auto_mode_state_shares_the_same_cell_as_a_clone() {
        // This is the property the CLI's approval callback relies on:
        // App::set_auto_mode_state must install the *same* Arc<Mutex<_>>
        // the callback was built with, not a copy - otherwise toggling
        // Shift+Tab in the TUI would never affect approval decisions.
        let mut app = test_app().await;
        let shared = Arc::new(Mutex::new(PlanExecMode::FullAuto));
        app.set_auto_mode_state(shared.clone());

        assert_eq!(app.auto_mode(), PlanExecMode::FullAuto);

        app.handle_key_event(key(KeyCode::BackTab)).await.unwrap();

        // The external handle must observe the change made through `app`.
        assert_eq!(*shared.lock().unwrap(), PlanExecMode::Interactive);
    }

    #[tokio::test]
    async fn slash_skills_command_opens_skills_view() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.working_directory = std::env::temp_dir();

        assert!(app.try_handle_slash_command("/skills").await.unwrap());

        assert_eq!(app.mode, AppMode::Skills);
    }

    #[tokio::test]
    async fn slash_mcp_command_opens_mcp_view() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.mcp_status = vec![crate::mcp::McpServerStatus {
            name: "test-server".to_string(),
            command: "echo".to_string(),
            connected: true,
            tool_count: 3,
            error: None,
        }];

        assert!(app.try_handle_slash_command("/mcp").await.unwrap());

        assert_eq!(app.mode, AppMode::Mcp);
    }

    #[tokio::test]
    async fn slash_help_command_opens_help_view() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        assert!(app.try_handle_slash_command("/help").await.unwrap());

        assert_eq!(app.mode, AppMode::Help);
    }

    #[tokio::test]
    async fn unrecognized_slash_word_falls_through_instead_of_being_swallowed() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        // A file path, not a command - must not be recognized/consumed.
        let handled = app
            .try_handle_slash_command("/usr/local/bin/cargo")
            .await
            .unwrap();

        assert!(!handled);
        assert_eq!(app.mode, AppMode::Chat);
    }

    /// Regression: ratatui-textarea underlines the cursor line by default, so
    /// everything typed into the chat input rendered underlined. All three
    /// paths that (re)build the textarea must clear that style - a fresh app,
    /// clearing the input after send, and the Plan Mode pre-fill.
    #[tokio::test]
    async fn chat_input_text_is_not_underlined() {
        use ratatui::style::Style;

        let mut app = test_app().await;
        assert_eq!(
            app.textarea.cursor_line_style(),
            Style::default(),
            "fresh input must not underline the cursor line"
        );

        app.clear_input();
        assert_eq!(
            app.textarea.cursor_line_style(),
            Style::default(),
            "clearing the input must not bring the underline back"
        );

        app.set_input_text("pre-filled revision request");
        assert_eq!(
            app.textarea.cursor_line_style(),
            Style::default(),
            "pre-filling the input must not bring the underline back"
        );
    }

    #[tokio::test]
    async fn non_slash_message_is_never_treated_as_a_command() {
        let mut app = test_app().await;
        let handled = app
            .try_handle_slash_command("just a normal message")
            .await
            .unwrap();
        assert!(!handled);
    }

    #[tokio::test]
    async fn typing_and_submitting_slash_skills_opens_the_dialog_end_to_end() {
        // Exercises the real path: typing characters into the textarea and
        // pressing Enter, not calling try_handle_slash_command directly.
        let mut app = test_app().await;
        app.mode = AppMode::Chat;
        app.working_directory = std::env::temp_dir();

        for c in "/skills".chars() {
            app.handle_chat_key(key(KeyCode::Char(c))).await.unwrap();
        }
        app.handle_chat_key(key(KeyCode::Enter)).await.unwrap();

        assert_eq!(app.mode, AppMode::Skills);
        // Must not have left the command text sitting in the input.
        assert!(app.textarea.is_empty());
    }

    #[tokio::test]
    async fn skills_view_up_down_navigation_clamps_at_bounds() {
        let mut app = test_app().await;
        app.mode = AppMode::Skills;
        app.skills_list = vec![
            crate::llm::tools::skill::SkillListing {
                name: "a".to_string(),
                description: None,
                root: std::path::PathBuf::new(),
            },
            crate::llm::tools::skill::SkillListing {
                name: "b".to_string(),
                description: None,
                root: std::path::PathBuf::new(),
            },
        ];

        app.handle_skills_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.skills_selected, 0);

        app.handle_skills_key(key(KeyCode::Down)).await.unwrap();
        app.handle_skills_key(key(KeyCode::Down)).await.unwrap();
        assert_eq!(app.skills_selected, 1);
    }

    #[tokio::test]
    async fn skills_view_esc_returns_to_chat() {
        let mut app = test_app().await;
        app.mode = AppMode::Skills;

        app.handle_skills_key(key(KeyCode::Esc)).await.unwrap();

        assert_eq!(app.mode, AppMode::Chat);
    }

    #[tokio::test]
    async fn mcp_view_up_down_navigation_clamps_at_bounds() {
        let mut app = test_app().await;
        app.mode = AppMode::Mcp;
        app.mcp_status = vec![
            crate::mcp::McpServerStatus {
                name: "a".to_string(),
                command: "cmd-a".to_string(),
                connected: true,
                tool_count: 1,
                error: None,
            },
            crate::mcp::McpServerStatus {
                name: "b".to_string(),
                command: "cmd-b".to_string(),
                connected: false,
                tool_count: 0,
                error: Some("failed to spawn".to_string()),
            },
        ];

        app.handle_mcp_key(key(KeyCode::Up)).await.unwrap();
        assert_eq!(app.mcp_selected, 0);

        app.handle_mcp_key(key(KeyCode::Down)).await.unwrap();
        app.handle_mcp_key(key(KeyCode::Down)).await.unwrap();
        assert_eq!(app.mcp_selected, 1);
    }

    #[tokio::test]
    async fn mcp_view_esc_returns_to_chat() {
        let mut app = test_app().await;
        app.mode = AppMode::Mcp;

        app.handle_mcp_key(key(KeyCode::Esc)).await.unwrap();

        assert_eq!(app.mode, AppMode::Chat);
    }

    #[tokio::test]
    async fn chat_plain_enter_submits_and_clears_buffer() {
        let mut app = test_app().await;
        app.handle_chat_key(key(KeyCode::Char('h'))).await.unwrap();
        app.handle_chat_key(key(KeyCode::Char('i'))).await.unwrap();

        // Plain Enter now sends (no session is set up in this harness, so
        // send_message() is a no-op, but the input buffer must still clear
        // as soon as submit is triggered).
        app.handle_chat_key(key(KeyCode::Enter)).await.unwrap();

        assert!(app.textarea.is_empty());
    }

    #[tokio::test]
    async fn chat_plain_enter_on_empty_buffer_does_nothing() {
        let mut app = test_app().await;

        app.handle_chat_key(key(KeyCode::Enter)).await.unwrap();

        assert!(app.textarea.is_empty());
        assert!(app.messages.is_empty());
    }

    #[tokio::test]
    async fn ctrl_o_opens_model_info_panel_and_esc_closes_it() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.handle_key_event(key_mod(KeyCode::Char('o'), KeyModifiers::CONTROL))
            .await
            .unwrap();
        assert_eq!(app.mode, AppMode::ModelInfo);

        app.handle_key_event(key(KeyCode::Esc)).await.unwrap();
        assert_eq!(app.mode, AppMode::Chat);
    }

    #[tokio::test]
    async fn last_assistant_message_finds_most_recent_assistant_reply() {
        let mut app = test_app().await;
        assert!(app.last_assistant_message().is_none());

        app.messages.push(DisplayMessage {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "hi".to_string(),
            thinking_text: None,
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: None,
            cost: None,
            provider_name: None,
            perf_metrics: None,
            tokens_per_second: None,
        });
        app.messages.push(DisplayMessage {
            id: Uuid::new_v4(),
            role: "assistant".to_string(),
            content: "first".to_string(),
            thinking_text: None,
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: None,
            cost: None,
            provider_name: None,
            perf_metrics: None,
            tokens_per_second: None,
        });
        app.messages.push(DisplayMessage {
            id: Uuid::new_v4(),
            role: "assistant".to_string(),
            content: "second".to_string(),
            thinking_text: None,
            thinking_expanded: false,
            timestamp: chrono::Utc::now(),
            token_count: None,
            cost: None,
            provider_name: None,
            perf_metrics: None,
            tokens_per_second: None,
        });

        assert_eq!(app.last_assistant_message().unwrap().content, "second");
    }

    #[tokio::test]
    async fn chat_ctrl_enter_still_submits_as_legacy_alias() {
        let mut app = test_app().await;
        app.handle_chat_key(key(KeyCode::Char('h'))).await.unwrap();

        app.handle_chat_key(key_mod(KeyCode::Enter, KeyModifiers::CONTROL))
            .await
            .unwrap();

        assert!(app.textarea.is_empty());
    }

    #[tokio::test]
    async fn ctrl_w_opens_provider_switch_dialog_in_loading_state() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.handle_key_event(key_mod(KeyCode::Char('w'), KeyModifiers::CONTROL))
            .await
            .unwrap();

        assert_eq!(app.mode, AppMode::ProviderSwitch);
        assert!(app.provider_switch_loading);
        assert!(app.provider_switch_models.is_empty());
    }

    #[tokio::test]
    async fn provider_switch_models_listed_clears_loading_state() {
        let mut app = test_app().await;
        app.provider_switch_loading = true;

        app.handle_event(TuiEvent::ProviderSwitchModelsListed(vec![
            "qwen2.5-coder:7b".to_string(),
            "llama3.2:3b".to_string(),
        ]))
        .await
        .unwrap();

        assert!(!app.provider_switch_loading);
        assert_eq!(app.provider_switch_models.len(), 2);
        assert_eq!(app.provider_switch_selected, 0);
    }

    #[tokio::test]
    async fn provider_switch_up_down_navigation_clamps_at_bounds() {
        let mut app = test_app().await;
        app.mode = AppMode::ProviderSwitch;
        app.provider_switch_loading = false;
        app.provider_switch_models = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        // Up from index 0 stays at 0.
        app.handle_provider_switch_key(key(KeyCode::Up))
            .await
            .unwrap();
        assert_eq!(app.provider_switch_selected, 0);

        app.handle_provider_switch_key(key(KeyCode::Down))
            .await
            .unwrap();
        app.handle_provider_switch_key(key(KeyCode::Down))
            .await
            .unwrap();
        assert_eq!(app.provider_switch_selected, 2);

        // Down at the last index stays at the last index.
        app.handle_provider_switch_key(key(KeyCode::Down))
            .await
            .unwrap();
        assert_eq!(app.provider_switch_selected, 2);
    }

    #[tokio::test]
    async fn provider_switch_esc_returns_to_chat() {
        let mut app = test_app().await;
        app.mode = AppMode::ProviderSwitch;
        app.provider_switch_loading = true;

        app.handle_provider_switch_key(key(KeyCode::Esc))
            .await
            .unwrap();

        assert_eq!(app.mode, AppMode::Chat);
    }

    #[cfg(not(feature = "ollama"))]
    #[tokio::test]
    async fn switch_provider_without_ollama_feature_shows_clear_error() {
        let mut app = test_app().await;
        app.mode = AppMode::ProviderSwitch;

        app.switch_provider_to_ollama_model("qwen2.5-coder:7b".to_string())
            .await
            .unwrap();

        assert!(app
            .error_message
            .as_ref()
            .is_some_and(|e| e.contains("--features ollama")));
        // Must not silently pretend to switch mode/post a success message.
        assert_eq!(app.mode, AppMode::ProviderSwitch);
    }

    #[cfg(feature = "ollama")]
    #[tokio::test]
    async fn switch_provider_with_ollama_feature_swaps_provider_in_place() {
        let mut app = test_app().await;
        app.mode = AppMode::ProviderSwitch;
        let original_provider_name = app.provider_name().to_string();

        app.switch_provider_to_ollama_model("qwen2.5-coder:7b".to_string())
            .await
            .unwrap();

        assert!(app.error_message.is_none());
        assert_eq!(app.mode, AppMode::Chat);
        assert_eq!(app.provider_name(), "ollama");
        assert_ne!(app.provider_name(), original_provider_name);
        assert_eq!(app.provider_model(), "qwen2.5-coder:7b");
        assert!(app
            .messages
            .iter()
            .any(|m| m.content.contains("Switched to Ollama model")));
    }

    #[tokio::test]
    async fn ctrl_g_opens_llama_cpp_models_dialog_in_loading_state() {
        let mut app = test_app().await;
        app.mode = AppMode::Chat;

        app.handle_key_event(key_mod(KeyCode::Char('g'), KeyModifiers::CONTROL))
            .await
            .unwrap();

        assert_eq!(app.mode, AppMode::LlamaCppModelPicker);
        assert!(app.llama_cpp_loading);
        assert!(app.llama_cpp_models.is_empty());
    }

    #[tokio::test]
    async fn llama_cpp_models_listed_clears_loading_state() {
        let mut app = test_app().await;
        app.llama_cpp_loading = true;

        app.handle_event(TuiEvent::LlamaCppModelsListed(vec![
            super::super::llama_cpp_download::LlamaCppModelSummary {
                path: std::path::PathBuf::from("/models/a.gguf"),
                size_bytes: 100,
                quantization_hint: Some("Q4_K_M".to_string()),
                architecture: None,
                parameter_count: None,
                context_length: None,
                has_chat_template: false,
                display_name: None,
                estimated_memory_bytes: None,
                estimated_memory_includes_kv_cache: false,
                is_mmproj: false,
                mmproj_path: None,
            },
            super::super::llama_cpp_download::LlamaCppModelSummary {
                path: std::path::PathBuf::from("/models/b.gguf"),
                size_bytes: 200,
                quantization_hint: None,
                architecture: None,
                parameter_count: None,
                context_length: None,
                has_chat_template: false,
                display_name: None,
                estimated_memory_bytes: None,
                estimated_memory_includes_kv_cache: false,
                is_mmproj: false,
                mmproj_path: None,
            },
        ]))
        .await
        .unwrap();

        assert!(!app.llama_cpp_loading);
        assert_eq!(app.llama_cpp_models.len(), 2);
        assert_eq!(app.llama_cpp_selected, 0);
    }

    #[tokio::test]
    async fn llama_cpp_up_down_navigation_clamps_at_bounds() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_loading = false;
        app.llama_cpp_models = vec![
            super::super::llama_cpp_download::LlamaCppModelSummary {
                path: std::path::PathBuf::from("/models/a.gguf"),
                size_bytes: 100,
                quantization_hint: None,
                architecture: None,
                parameter_count: None,
                context_length: None,
                has_chat_template: false,
                display_name: None,
                estimated_memory_bytes: None,
                estimated_memory_includes_kv_cache: false,
                is_mmproj: false,
                mmproj_path: None,
            },
            super::super::llama_cpp_download::LlamaCppModelSummary {
                path: std::path::PathBuf::from("/models/b.gguf"),
                size_bytes: 100,
                quantization_hint: None,
                architecture: None,
                parameter_count: None,
                context_length: None,
                has_chat_template: false,
                display_name: None,
                estimated_memory_bytes: None,
                estimated_memory_includes_kv_cache: false,
                is_mmproj: false,
                mmproj_path: None,
            },
        ];

        app.handle_llama_cpp_models_key(key(KeyCode::Up))
            .await
            .unwrap();
        assert_eq!(app.llama_cpp_selected, 0);

        app.handle_llama_cpp_models_key(key(KeyCode::Down))
            .await
            .unwrap();
        app.handle_llama_cpp_models_key(key(KeyCode::Down))
            .await
            .unwrap();
        assert_eq!(app.llama_cpp_selected, 1);
    }

    #[tokio::test]
    async fn llama_cpp_esc_returns_to_chat() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_loading = true;

        app.handle_llama_cpp_models_key(key(KeyCode::Esc))
            .await
            .unwrap();

        assert_eq!(app.mode, AppMode::Chat);
    }

    #[tokio::test]
    async fn llama_cpp_typing_fills_the_download_input() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_loading = false;

        app.handle_llama_cpp_models_key(key(KeyCode::Char('h')))
            .await
            .unwrap();
        app.handle_llama_cpp_models_key(key(KeyCode::Char('f')))
            .await
            .unwrap();

        assert_eq!(app.llama_cpp_download_input, "hf");
    }

    #[tokio::test]
    async fn llama_cpp_delete_key_asks_for_confirmation_before_deleting() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_models = vec![super::super::llama_cpp_download::LlamaCppModelSummary {
            path: std::path::PathBuf::from("/models/a.gguf"),
            size_bytes: 100,
            quantization_hint: None,
            architecture: None,
            parameter_count: None,
            context_length: None,
            has_chat_template: false,
            display_name: None,
            estimated_memory_bytes: None,
            estimated_memory_includes_kv_cache: false,
            is_mmproj: false,
            mmproj_path: None,
        }];

        app.handle_llama_cpp_models_key(key(KeyCode::Delete))
            .await
            .unwrap();

        assert_eq!(
            app.llama_cpp_confirm_delete,
            Some(std::path::PathBuf::from("/models/a.gguf"))
        );
        // Nothing deleted yet - only asked.
        assert_eq!(app.llama_cpp_models.len(), 1);
    }

    #[tokio::test]
    async fn llama_cpp_delete_finished_removes_model_from_list() {
        let mut app = test_app().await;
        app.llama_cpp_models = vec![super::super::llama_cpp_download::LlamaCppModelSummary {
            path: std::path::PathBuf::from("/models/a.gguf"),
            size_bytes: 100,
            quantization_hint: None,
            architecture: None,
            parameter_count: None,
            context_length: None,
            has_chat_template: false,
            display_name: None,
            estimated_memory_bytes: None,
            estimated_memory_includes_kv_cache: false,
            is_mmproj: false,
            mmproj_path: None,
        }];
        app.llama_cpp_deleting = Some(std::path::PathBuf::from("/models/a.gguf"));

        app.handle_event(TuiEvent::LlamaCppDeleteFinished {
            path: std::path::PathBuf::from("/models/a.gguf"),
            error: None,
        })
        .await
        .unwrap();

        assert!(app.llama_cpp_deleting.is_none());
        assert!(app.llama_cpp_models.is_empty());
        assert_eq!(app.mode, AppMode::Chat);
        assert!(app.messages.iter().any(|m| m.content.contains("Deleted")));
    }

    /// The freshly-built provider crosses the switch task's thread boundary
    /// through `llama_cpp_pending_provider`, not `TuiEvent` (which can't
    /// carry a `Provider` trait object - see the field's own doc comment).
    /// Simulating that here (rather than going through the real,
    /// feature-gated `spawn_switch`) keeps this test exercising the same
    /// event-driven swap-in logic the real path uses, without needing an
    /// actual `.gguf` file.
    #[tokio::test]
    async fn llama_cpp_switch_finished_swaps_provider_in_place() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_switching = Some(std::path::PathBuf::from("/models/a.gguf"));
        *app.llama_cpp_pending_provider.lock().unwrap() =
            Some(Arc::new(DummyProvider) as Arc<dyn Provider>);

        app.handle_event(TuiEvent::LlamaCppSwitchFinished {
            model_path: std::path::PathBuf::from("/models/a.gguf"),
            error: None,
        })
        .await
        .unwrap();

        assert!(app.llama_cpp_switching.is_none());
        assert_eq!(app.mode, AppMode::Chat);
        assert_eq!(app.provider_name(), "dummy");
        assert!(app
            .messages
            .iter()
            .any(|m| m.content.contains("Switched to")));
    }

    #[tokio::test]
    async fn llama_cpp_switch_finished_with_error_reports_failure_without_swapping() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_switching = Some(std::path::PathBuf::from("/models/a.gguf"));
        let original_provider_name = app.provider_name().to_string();

        app.handle_event(TuiEvent::LlamaCppSwitchFinished {
            model_path: std::path::PathBuf::from("/models/a.gguf"),
            error: Some("model file not found".to_string()),
        })
        .await
        .unwrap();

        assert!(app.llama_cpp_switching.is_none());
        assert_eq!(app.provider_name(), original_provider_name);
        assert!(app
            .messages
            .iter()
            .any(|m| m.content.contains("Failed to load")));
    }

    /// A switch, download, and delete all share one dialog - only one
    /// operation may run at a time. Enter must not start a second switch
    /// while a delete is already in flight.
    #[tokio::test]
    async fn llama_cpp_switch_is_a_noop_while_a_delete_is_already_running() {
        let mut app = test_app().await;
        app.mode = AppMode::LlamaCppModelPicker;
        app.llama_cpp_deleting = Some(std::path::PathBuf::from("/models/a.gguf"));
        app.llama_cpp_models = vec![super::super::llama_cpp_download::LlamaCppModelSummary {
            path: std::path::PathBuf::from("/models/b.gguf"),
            size_bytes: 100,
            quantization_hint: None,
            architecture: None,
            parameter_count: None,
            context_length: None,
            has_chat_template: false,
            display_name: None,
            estimated_memory_bytes: None,
            estimated_memory_includes_kv_cache: false,
            is_mmproj: false,
            mmproj_path: None,
        }];

        // Enter would normally start a switch to the highlighted model.
        app.handle_llama_cpp_models_key(key(KeyCode::Enter))
            .await
            .unwrap();

        assert!(app.llama_cpp_switching.is_none());
    }

    /// Regression: `send_message` spawns the agent call as a detached
    /// background task; its `ResponseChunk`/`ResponseComplete` events used
    /// to be applied unconditionally to whatever session was current when
    /// they arrived. If the user switched sessions while a request was
    /// still in flight, session A's streamed reply got appended to session
    /// B's in-memory transcript. Events are now tagged with the session
    /// they were made against and dropped if that session is no longer
    /// current.
    #[tokio::test]
    async fn stale_session_response_chunk_is_dropped_after_switching_sessions() {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let session_a_id = app.current_session.as_ref().unwrap().id;

        // Switch to a different session while "session A's" request is
        // still notionally in flight.
        app.create_new_session().await.unwrap();
        let session_b_id = app.current_session.as_ref().unwrap().id;
        assert_ne!(session_a_id, session_b_id);

        app.handle_event(TuiEvent::ResponseChunk(
            session_a_id,
            "stale chunk from session A".to_string(),
        ))
        .await
        .unwrap();

        assert!(
            app.streaming_response.is_none(),
            "a chunk tagged with a session that is no longer current must not be applied: {:?}",
            app.streaming_response
        );

        // A chunk tagged with the *current* session must still be applied.
        app.handle_event(TuiEvent::ResponseChunk(
            session_b_id,
            "live chunk from session B".to_string(),
        ))
        .await
        .unwrap();

        assert_eq!(
            app.streaming_response.as_deref(),
            Some("live chunk from session B")
        );
    }

    /// Regression: an agent error mid-plan-execution used to leave the
    /// in-progress task stuck forever with `executing_plan` still `true`
    /// and no further task ever dispatched. The error must instead mark
    /// the task `Failed` and stop auto-execution so the plan isn't left in
    /// a silently frozen state.
    #[tokio::test]
    async fn plan_task_error_marks_task_failed_and_stops_auto_execution() {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let session_id = app.current_session.as_ref().unwrap().id;

        let mut plan = crate::plan::PlanDocument::new(
            session_id,
            "Test plan".to_string(),
            "A plan".to_string(),
        );
        let mut task = crate::plan::PlanTask::new(
            1,
            "Do the thing".to_string(),
            "Description".to_string(),
            crate::plan::TaskType::Edit,
        );
        task.status = crate::plan::TaskStatus::InProgress;
        plan.add_task(task);
        app.current_plan = Some(plan);
        app.executing_plan = true;

        app.handle_event(TuiEvent::Error(
            session_id,
            "provider timed out".to_string(),
        ))
        .await
        .unwrap();

        assert!(
            !app.executing_plan,
            "auto-execution must stop after an agent error"
        );
        let task = &app.current_plan.as_ref().unwrap().tasks[0];
        assert!(
            matches!(task.status, crate::plan::TaskStatus::Failed),
            "the in-progress task must be marked Failed, got {:?}",
            task.status
        );
        assert!(app
            .error_message
            .as_ref()
            .is_some_and(|m| m.contains("provider timed out")));
    }

    /// Same guard, for the completed-response path: a stale session's
    /// completed response must not be appended to a different session's
    /// message list, and must not clear `is_processing` for a request the
    /// user is still actively waiting on in the current session.
    #[tokio::test]
    async fn stale_session_response_complete_is_dropped_after_switching_sessions() {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let session_a_id = app.current_session.as_ref().unwrap().id;

        app.create_new_session().await.unwrap();
        let session_b_id = app.current_session.as_ref().unwrap().id;
        let messages_before = app.messages.len();
        // Session B has its own genuine in-flight request - distinct from
        // session A's stale one, which the event below carries.
        app.is_processing = true;
        app.processing_session = Some(session_b_id);

        let stale_response = crate::llm::agent::AgentResponse {
            message_id: Uuid::new_v4(),
            content: "reply that belongs to session A".to_string(),
            thinking_text: None,
            stop_reason: None,
            usage: crate::llm::provider::TokenUsage {
                input_tokens: 1,
                output_tokens: 1,
            },
            cost: 0.0,
            model: "test-model".to_string(),
            provider_name: "test-provider".to_string(),
            perf_metrics: None,
        };

        app.handle_event(TuiEvent::ResponseComplete(session_a_id, stale_response))
            .await
            .unwrap();

        assert_eq!(
            app.messages.len(),
            messages_before,
            "a stale session's completed response must not be appended to the current session"
        );
        assert!(
            app.is_processing,
            "a stale session's completion must not clear is_processing for the current session's own in-flight request"
        );
    }

    /// Regression: this is the actual bug the earlier tests above didn't
    /// catch (they only checked that a stale event doesn't corrupt an
    /// *already-correct* is_processing value, not that switching sessions
    /// itself sets that value correctly). Switching away from a session
    /// mid-request used to leave `is_processing` stuck `true` and
    /// `streaming_response` frozen on the abandoned reply forever: nothing
    /// reset them for the newly-current session, since `complete_response`/
    /// `show_error` only run for the session that's current *at the time
    /// its own event arrives*, and the stale event that would eventually
    /// arrive for the abandoned session is correctly dropped without
    /// calling either.
    #[tokio::test]
    async fn switching_sessions_clears_a_stuck_processing_state_from_the_previous_session() {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let session_a_id = app.current_session.as_ref().unwrap().id;

        // Simulate a request in flight for session A.
        app.is_processing = true;
        app.processing_session = Some(session_a_id);
        app.streaming_response = Some("partial reply for A".to_string());

        // User switches away before the request resolves.
        app.create_new_session().await.unwrap();
        let session_b_id = app.current_session.as_ref().unwrap().id;
        assert_ne!(session_a_id, session_b_id);

        assert!(
            !app.is_processing,
            "switching to a session with nothing in flight must clear is_processing, \
             not leave it stuck from the abandoned session"
        );
        assert!(
            app.streaming_response.is_none(),
            "switching away must not leave a different session's partial reply visible"
        );
    }

    /// Regression: pressing Ctrl+K (clear session) while a response is still
    /// generating used to delete every message for the session out from under
    /// the detached agent task, whose trailing `update_message_usage` then
    /// failed with "Message not found". `clear_session` must instead refuse
    /// and leave the transcript untouched while a request is in flight for the
    /// current session.
    #[tokio::test]
    async fn clear_session_is_refused_while_the_current_session_is_processing() {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let session_id = app.current_session.as_ref().unwrap().id;

        // Persist a message so we can prove nothing was deleted.
        app.message_service
            .create_message(session_id, "assistant".to_string(), "in flight".to_string())
            .await
            .unwrap();

        // A request is in flight for THIS session.
        app.is_processing = true;
        app.processing_session = Some(session_id);

        app.clear_session().await.unwrap();

        let remaining = app
            .message_service
            .list_messages_for_session(session_id)
            .await
            .unwrap();
        assert_eq!(
            remaining.len(),
            1,
            "clear_session must not delete messages while the session is processing"
        );
        assert!(
            app.error_message
                .as_ref()
                .is_some_and(|m| m.contains("Wait for the response")),
            "the user should see a hint explaining why Ctrl+K did nothing"
        );
    }

    /// The guard is scoped to the *current* session: a request in flight for a
    /// different session must not block clearing the one on screen.
    #[tokio::test]
    async fn clear_session_proceeds_when_only_another_session_is_processing() {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let other_session_id = app.current_session.as_ref().unwrap().id;

        app.create_new_session().await.unwrap();
        let current_session_id = app.current_session.as_ref().unwrap().id;
        assert_ne!(other_session_id, current_session_id);

        app.message_service
            .create_message(
                current_session_id,
                "assistant".to_string(),
                "on screen".to_string(),
            )
            .await
            .unwrap();

        // Something is processing, but for the OTHER session.
        app.is_processing = true;
        app.processing_session = Some(other_session_id);

        app.clear_session().await.unwrap();

        let remaining = app
            .message_service
            .list_messages_for_session(current_session_id)
            .await
            .unwrap();
        assert!(
            remaining.is_empty(),
            "clear_session must proceed when the in-flight request belongs to a different session"
        );
    }

    /// The other half of the same fix: switching *back* to a session whose
    /// request genuinely is still outstanding must correctly show the
    /// processing state again, rather than clearing it unconditionally.
    #[tokio::test]
    async fn switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state()
    {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let session_a_id = app.current_session.as_ref().unwrap().id;
        app.is_processing = true;
        app.processing_session = Some(session_a_id);
        app.streaming_response = Some("partial reply for A".to_string());

        app.create_new_session().await.unwrap();
        assert!(
            !app.is_processing,
            "sanity: session B has nothing in flight"
        );

        app.load_session(session_a_id).await.unwrap();

        assert!(
            app.is_processing,
            "switching back to a session whose request is still outstanding \
             must show the processing state again"
        );
    }

    /// Regression: `send_message` never checked `is_processing` before
    /// spawning a new agent call, so pressing Enter again while a response
    /// was still streaming spawned a second concurrent
    /// `send_message_with_tools_and_mode_streaming` call against the same
    /// session - racing the first for message ordering and DB writes.
    #[tokio::test]
    async fn send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight() {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let session_id = app.current_session.as_ref().unwrap().id;
        app.is_processing = true;
        app.processing_session = Some(session_id);
        let messages_before = app.messages.len();

        app.send_message("second submission while streaming".to_string())
            .await
            .unwrap();

        assert_eq!(
            app.messages.len(),
            messages_before,
            "a second submission for the same in-flight session must not \
             append another user message or spawn another agent call"
        );
    }

    /// The guard must be scoped to *the current session's* in-flight
    /// request, not global - switching to a fresh session while another
    /// session is still processing must still allow sending.
    #[tokio::test]
    async fn send_message_still_works_for_a_different_session_than_the_one_processing() {
        let mut app = test_app().await;
        app.create_new_session().await.unwrap();
        let session_a_id = app.current_session.as_ref().unwrap().id;
        app.is_processing = true;
        app.processing_session = Some(session_a_id);

        app.create_new_session().await.unwrap();
        let messages_before = app.messages.len();

        app.send_message("hello from session B".to_string())
            .await
            .unwrap();

        assert_eq!(
            app.messages.len(),
            messages_before + 1,
            "session B has nothing in flight, so its own submission must go through"
        );
    }
}
