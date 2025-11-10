//! TUI Application State
//!
//! Core state management for the terminal user interface.

use super::events::{AppMode, EventHandler, ToolApprovalRequest, ToolApprovalResponse, TuiEvent};
use crate::db::models::{Message, Session};
use crate::llm::agent::AgentService;
use crate::services::{MessageService, ServiceContext, SessionService};
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

/// Display message for UI rendering
#[derive(Debug, Clone)]
pub struct DisplayMessage {
    pub id: Uuid,
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub token_count: Option<i32>,
    pub cost: Option<f64>,
}

impl From<Message> for DisplayMessage {
    fn from(msg: Message) -> Self {
        Self {
            id: msg.id,
            role: msg.role,
            content: msg.content,
            timestamp: msg.created_at,
            token_count: msg.token_count,
            cost: msg.cost,
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
    pub input_buffer: String,
    pub scroll_offset: usize,
    pub selected_session_index: usize,
    pub should_quit: bool,

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

    // Services
    agent_service: Arc<AgentService>,
    session_service: SessionService,
    message_service: MessageService,

    // Events
    event_handler: EventHandler,
}

impl App {
    /// Create a new app instance
    pub fn new(agent_service: Arc<AgentService>, context: ServiceContext) -> Self {
        Self {
            current_session: None,
            messages: Vec::new(),
            sessions: Vec::new(),
            mode: AppMode::Splash,
            input_buffer: String::new(),
            scroll_offset: 0,
            selected_session_index: 0,
            should_quit: false,
            is_processing: false,
            streaming_response: None,
            error_message: None,
            animation_frame: 0,
            splash_shown_at: Some(std::time::Instant::now()),
            pending_approval: None,
            show_approval_details: false,
            session_service: SessionService::new(context.clone()),
            message_service: MessageService::new(context),
            agent_service,
            event_handler: EventHandler::new(),
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
            }
            TuiEvent::Resize(_, _) | TuiEvent::AgentProcessing => {
                // These are handled by the render loop
            }
        }
        Ok(())
    }

    /// Handle keyboard input
    async fn handle_key_event(&mut self, event: crossterm::event::KeyEvent) -> Result<()> {
        use super::events::keys;

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

        // Mode-specific handling
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
            AppMode::Sessions => self.handle_sessions_key(event).await?,
            AppMode::ToolApproval => self.handle_approval_key(event).await?,
            AppMode::Help | AppMode::Settings => {
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
        use crossterm::event::KeyCode;

        if keys::is_submit(&event) && !self.input_buffer.trim().is_empty() {
            let content = self.input_buffer.clone();
            self.input_buffer.clear();
            self.send_message(content).await?;
        } else if keys::is_cancel(&event) {
            self.input_buffer.clear();
            self.error_message = None;
        } else if keys::is_page_up(&event) {
            self.scroll_offset = self.scroll_offset.saturating_sub(10);
        } else if keys::is_page_down(&event) {
            self.scroll_offset = self.scroll_offset.saturating_add(10);
        } else {
            // Regular character input
            match event.code {
                KeyCode::Char(c) => {
                    self.input_buffer.push(c);
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Enter => {
                    self.input_buffer.push('\n');
                }
                _ => {}
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

    /// Send a message to the agent
    async fn send_message(&mut self, content: String) -> Result<()> {
        if let Some(session) = &self.current_session {
            self.is_processing = true;
            self.error_message = None;

            // Add user message to UI immediately
            let user_msg = DisplayMessage {
                id: Uuid::new_v4(),
                role: "user".to_string(),
                content: content.clone(),
                timestamp: chrono::Utc::now(),
                token_count: None,
                cost: None,
            };
            self.messages.push(user_msg);

            // Send to agent in background
            let agent_service = self.agent_service.clone();
            let session_id = session.id;
            let event_sender = self.event_sender();

            tokio::spawn(async move {
                match agent_service
                    .send_message_with_tools(session_id, content, None)
                    .await
                {
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
        }
    }

    /// Complete the streaming response
    async fn complete_response(
        &mut self,
        response: crate::llm::agent::AgentResponse,
    ) -> Result<()> {
        self.is_processing = false;
        self.streaming_response = None;

        // Add assistant message to UI
        let assistant_msg = DisplayMessage {
            id: response.message_id,
            role: "assistant".to_string(),
            content: response.content,
            timestamp: chrono::Utc::now(),
            token_count: Some(
                response.usage.input_tokens as i32 + response.usage.output_tokens as i32,
            ),
            cost: Some(response.cost),
        };
        self.messages.push(assistant_msg);

        // Auto-scroll to bottom
        self.scroll_offset = 0;

        Ok(())
    }

    /// Show an error message
    fn show_error(&mut self, error: String) {
        self.is_processing = false;
        self.streaming_response = None;
        self.error_message = Some(error);
    }

    /// Switch to a different mode
    async fn switch_mode(&mut self, mode: AppMode) -> Result<()> {
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
        };

        let display_msg: DisplayMessage = msg.into();
        assert_eq!(display_msg.role, "user");
        assert_eq!(display_msg.content, "Hello");
    }
}
