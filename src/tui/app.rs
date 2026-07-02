//! TUI Application State
//!
//! Core state management for the terminal user interface.

use super::events::{AppMode, EventHandler, ToolApprovalRequest, ToolApprovalResponse, TuiEvent};
use super::plan::PlanDocument;
use super::prompt_analyzer::PromptAnalyzer;
use crate::db::models::{Message, Session};
use crate::llm::agent::AgentService;
use crate::services::{MessageService, PlanService, ServiceContext, SessionService};
use anyhow::Result;
use std::sync::Arc;
use tui_textarea::{CursorMove, TextArea};
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
    pub scroll_offset: usize,
    pub selected_session_index: usize,
    pub should_quit: bool,
    /// Whether the terminal supports the Kitty keyboard enhancement
    /// protocol (needed to disambiguate `Shift+Enter` from plain `Enter`).
    /// Set once at startup by the runner; only affects which key hints are
    /// shown, not which keys are actually handled (`Alt+Enter` always works
    /// as a newline fallback regardless of this flag).
    pub kitty_keyboard_protocol_active: bool,

    // Streaming state
    pub is_processing: bool,
    pub streaming_response: Option<String>,
    pub error_message: Option<String>,

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
    ollama_host: String,

    // Provider Switch dialog state (Ctrl+W, native Ollama provider)
    pub provider_switch_models: Vec<String>,
    pub provider_switch_selected: usize,
    pub provider_switch_loading: bool,

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

impl App {
    /// Create a new app instance
    pub fn new(agent_service: Arc<AgentService>, context: ServiceContext) -> Self {
        Self {
            current_session: None,
            messages: Vec::new(),
            sessions: Vec::new(),
            mode: AppMode::Splash,
            textarea: TextArea::default(),
            scroll_offset: 0,
            selected_session_index: 0,
            should_quit: false,
            kitty_keyboard_protocol_active: false,
            is_processing: false,
            streaming_response: None,
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
            ollama_host: "http://localhost:11434".to_string(),
            provider_switch_models: Vec::new(),
            provider_switch_selected: 0,
            provider_switch_loading: false,
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
        self.textarea = TextArea::default();
    }

    /// Replace the chat input's entire contents with `text` (used for the
    /// Plan Mode revision-request pre-fill, which overwrites rather than
    /// appends).
    fn set_input_text(&mut self, text: &str) {
        self.textarea = TextArea::default();
        self.textarea.insert_str(text);
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

    /// Record whether the terminal supports the Kitty keyboard enhancement
    /// protocol, detected once at startup by the runner.
    pub fn set_kitty_keyboard_protocol_active(&mut self, active: bool) {
        self.kitty_keyboard_protocol_active = active;
    }

    /// Receive next event
    pub async fn next_event(&mut self) -> Option<TuiEvent> {
        self.event_handler.next().await
    }

    /// Handle an event
    pub async fn handle_event(&mut self, event: TuiEvent) -> Result<()> {
        match event {
            TuiEvent::Key(key_event) => {
                self.handle_key_event(key_event).await?;
            }
            TuiEvent::Paste(text) => {
                // Handle paste events - only in Chat mode. Inserted at the
                // cursor position rather than blindly appended.
                if self.mode == AppMode::Chat {
                    self.textarea.insert_str(&text);
                }
            }
            TuiEvent::MessageSubmitted(content) => {
                self.send_message(content).await?;
            }
            TuiEvent::ResponseChunk(chunk) => {
                self.append_streaming_chunk(chunk);
            }
            TuiEvent::ResponseComplete(response) => {
                self.complete_response(response).await?;
            }
            TuiEvent::Error(error) => {
                self.show_error(error);
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
            self.send_message(content).await?;
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
        } else {
            let ctrl = event.modifiers.contains(KeyModifiers::CONTROL);
            match event.code {
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
        let session = self
            .session_service
            .create_session(Some("New Chat".to_string()))
            .await?;

        self.current_session = Some(session.clone());
        self.messages.clear();
        self.scroll_offset = 0;
        self.mode = AppMode::Chat;

        // Reload sessions list
        self.load_sessions().await?;

        Ok(())
    }

    /// Load a session and its messages
    async fn load_session(&mut self, session_id: Uuid) -> Result<()> {
        let session = self
            .session_service
            .get_session(session_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        let messages = self
            .message_service
            .list_messages_for_session(session_id)
            .await?;

        self.current_session = Some(session);
        self.messages = messages.into_iter().map(DisplayMessage::from).collect();
        self.scroll_offset = 0;

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
        if let Some(session) = &self.current_session {
            self.is_processing = true;
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
                    let _ = event_sender_chunks.send(TuiEvent::ResponseChunk(chunk));
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
                        let _ = event_sender.send(TuiEvent::ResponseComplete(response));
                    }
                    Err(e) => {
                        let _ = event_sender.send(TuiEvent::Error(e.to_string()));
                    }
                }
            });
        }

        Ok(())
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
            .find(|t| matches!(t.status, crate::tui::plan::TaskStatus::InProgress))
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
                    task.status = crate::tui::plan::TaskStatus::Failed;
                    task.notes = Some(
                        "Task failed during execution. Error detected in response.".to_string(),
                    );
                    true // Task failed
                } else {
                    // Mark task as completed successfully
                    task.status = crate::tui::plan::TaskStatus::Completed;
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
                match serde_json::from_str::<crate::tui::plan::PlanDocument>(&content) {
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
                if plan.status == crate::tui::plan::PlanStatus::PendingApproval {
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
                match serde_json::from_str::<crate::tui::plan::PlanDocument>(&content) {
                    Ok(plan) => {
                        tracing::debug!(
                            "Parsed plan: id={}, status={:?}, tasks={}",
                            plan.id,
                            plan.status,
                            plan.tasks.len()
                        );
                        // Only load if plan is pending approval
                        if plan.status == crate::tui::plan::PlanStatus::PendingApproval {
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

            markdown.push_str(&format!(
                "\n*Plan created: {}*\n",
                plan.created_at.format("%Y-%m-%d %H:%M:%S")
            ));
            markdown.push_str(&format!(
                "*Last updated: {}*\n",
                plan.updated_at.format("%Y-%m-%d %H:%M:%S")
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
                .find(|task| matches!(task.status, crate::tui::plan::TaskStatus::Pending))
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
                        task_mut.status = crate::tui::plan::TaskStatus::InProgress;
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

    /// Show an error message
    fn show_error(&mut self, error: String) {
        self.is_processing = false;
        self.streaming_response = None;
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

    /// Handle keys in the Model Download dialog.
    async fn handle_model_download_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;
        use crossterm::event::KeyCode;

        if keys::is_cancel(&event) {
            // Cancel an in-flight pull (if any) and close the dialog.
            if let Some(handle) = self.model_download_task.take() {
                handle.abort();
            }
            self.model_download_running = false;
            self.model_download_status = None;
            self.model_download_fraction = None;
            self.switch_mode(AppMode::Chat).await?;
            return Ok(());
        }

        // While a pull is running, only Esc (handled above) does anything.
        if self.model_download_running {
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
        match super::ollama_download::build_ollama_provider(&self.ollama_host, &model) {
            Ok(provider) => match Arc::get_mut(&mut self.agent_service) {
                Some(service) => {
                    service.set_provider(provider);
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
}
