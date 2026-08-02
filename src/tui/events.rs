//! TUI Event System
//!
//! Handles user input and application events for the terminal interface.

use crate::llm::agent::AgentResponse;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde_json::Value;
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Events that can occur in the TUI
#[derive(Debug, Clone)]
pub enum TuiEvent {
    /// User pressed a key
    Key(KeyEvent),

    /// User pasted text
    Paste(String),

    /// Terminal was resized
    Resize(u16, u16),

    /// User submitted a message
    MessageSubmitted(String),

    /// Agent started processing
    AgentProcessing,

    /// Agent sent a response chunk (streaming), tagged with the session the
    /// request was made against. `send_message` spawns the agent call as a
    /// detached background task; if the user switches sessions (or starts a
    /// second request) before it resolves, the event loop must be able to
    /// tell "this chunk belongs to a session that isn't on screen anymore"
    /// and drop it, instead of appending a stale request's output into
    /// whatever session happens to be current when the event arrives.
    ResponseChunk(Uuid, String),

    /// Agent completed response, tagged with the originating session (see
    /// `ResponseChunk`).
    ResponseComplete(Uuid, AgentResponse),

    /// An error occurred while handling a message for the given session
    /// (see `ResponseChunk`).
    Error(Uuid, String),

    /// Request to switch UI mode
    SwitchMode(AppMode),

    /// Request to select a session
    SelectSession(Uuid),

    /// Request to create new session
    NewSession,

    /// Request to quit
    Quit,

    /// Tick event for animations/updates
    Tick,

    /// Tool approval requested
    ToolApprovalRequested(ToolApprovalRequest),

    /// Tool approval response
    ToolApprovalResponse(ToolApprovalResponse),

    /// Locally-installed Ollama models were (re)loaded for the Model
    /// Download dialog's suggestion list.
    OllamaModelsListed(Vec<String>),

    /// Progress update for an in-flight Ollama model pull.
    OllamaPullProgress(super::ollama_download::ModelPullProgress),

    /// An Ollama model pull finished (`error` is `None` on success).
    OllamaPullFinished {
        model: String,
        error: Option<String>,
    },

    /// An Ollama model delete finished (`error` is `None` on success).
    OllamaDeleteFinished {
        model: String,
        error: Option<String>,
    },

    /// Locally-installed Ollama models were (re)loaded for the Provider
    /// Switch dialog's model list.
    ProviderSwitchModelsListed(Vec<String>),

    /// Local `.gguf` files were (re)scanned for the llama.cpp Local Models
    /// dialog (Ctrl+G).
    LlamaCppModelsListed(Vec<super::llama_cpp_download::LlamaCppModelSummary>),

    /// Progress update for an in-flight llama.cpp `.gguf` download.
    LlamaCppDownloadProgress(super::llama_cpp_download::LlamaCppDownloadProgress),

    /// A llama.cpp `.gguf` download finished (`error` is `None` on success).
    LlamaCppDownloadFinished {
        source: String,
        error: Option<String>,
    },

    /// A llama.cpp local model delete finished (`error` is `None` on success).
    LlamaCppDeleteFinished {
        path: std::path::PathBuf,
        error: Option<String>,
    },

    /// A llama.cpp model switch (loading a new `.gguf` as the active
    /// provider) finished (`error` is `None` on success). The freshly-built
    /// provider itself, on success, is left in `App`'s pending-provider slot
    /// rather than carried here - see `llama_cpp_download::PendingProvider`.
    LlamaCppSwitchFinished {
        model_path: std::path::PathBuf,
        error: Option<String>,
    },
}

/// Tool approval request details
#[derive(Debug, Clone)]
pub struct ToolApprovalRequest {
    /// Unique ID for this approval request
    pub request_id: Uuid,

    /// Tool name
    pub tool_name: String,

    /// Tool description
    pub tool_description: String,

    /// Tool input parameters
    pub tool_input: Value,

    /// Tool capabilities
    pub capabilities: Vec<String>,

    /// Channel to send response back
    pub response_tx: mpsc::UnboundedSender<ToolApprovalResponse>,

    /// When this request was created (for timeout)
    pub requested_at: std::time::Instant,
}

impl ToolApprovalRequest {
    /// Check if this approval request has timed out (default: 5 minutes)
    pub fn is_timed_out(&self) -> bool {
        self.requested_at.elapsed() > std::time::Duration::from_secs(300)
    }

    /// Get remaining time before timeout
    pub fn time_remaining(&self) -> std::time::Duration {
        let timeout = std::time::Duration::from_secs(300);
        let elapsed = self.requested_at.elapsed();
        timeout.saturating_sub(elapsed)
    }
}

/// Tool approval response
#[derive(Debug, Clone)]
pub struct ToolApprovalResponse {
    /// Request ID this is responding to
    pub request_id: Uuid,

    /// Whether the user approved
    pub approved: bool,

    /// Optional reason for denial
    pub reason: Option<String>,
}

/// Application mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// Splash screen
    Splash,
    /// Main chat interface (full execution)
    Chat,
    /// Plan mode (read-only, planning phase)
    Plan,
    /// Session list/management
    Sessions,
    /// Help screen
    Help,
    /// Settings
    Settings,
    /// Tool approval dialog
    ToolApproval,
    /// File picker dialog (triggered by @)
    FilePicker,
    /// Model download dialog (triggered by Ctrl+D) - pick/type an Ollama
    /// model name and pull it without leaving Crustly.
    ModelDownload,
    /// Model Info panel (triggered by Ctrl+O) - shows the active
    /// provider/model, context window, and last response's performance
    /// metrics.
    ModelInfo,
    /// Provider/model quick-switch dialog (triggered by Ctrl+W) - pick a
    /// locally-installed Ollama model and switch to it without restarting.
    ProviderSwitch,
    /// `/skills` slash command - lists every discoverable skill (project-
    /// local and user-global).
    Skills,
    /// `/mcp` slash command - lists configured MCP servers and their
    /// connection status.
    Mcp,
    /// llama.cpp Local Models dialog (triggered by Ctrl+G) - pick a
    /// locally-present `.gguf` file to switch to, or download a new one by
    /// URL/`hf:org/repo/file.gguf` shorthand.
    LlamaCppModelPicker,
}

/// Event handler for the TUI
pub struct EventHandler {
    /// Event sender
    tx: mpsc::UnboundedSender<TuiEvent>,

    /// Event receiver
    rx: mpsc::UnboundedReceiver<TuiEvent>,
}

impl EventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { tx, rx }
    }

    /// Get a sender for sending events
    pub fn sender(&self) -> mpsc::UnboundedSender<TuiEvent> {
        self.tx.clone()
    }

    /// Receive the next event
    /// Take an already-queued event without waiting, or `None` if the queue is
    /// empty. Lets the render loop drain a burst - a paste arriving as one key
    /// event per character, say - and repaint once for the whole batch instead
    /// of once per keystroke.
    pub fn try_next(&mut self) -> Option<TuiEvent> {
        self.rx.try_recv().ok()
    }

    pub async fn next(&mut self) -> Option<TuiEvent> {
        self.rx.recv().await
    }

    /// Start listening for terminal events
    pub fn start_terminal_listener(tx: mpsc::UnboundedSender<TuiEvent>) {
        tokio::spawn(async move {
            loop {
                // Poll for crossterm events with timeout
                if crossterm::event::poll(Duration::from_millis(100)).unwrap_or(false) {
                    if let Ok(event) = crossterm::event::read() {
                        match event {
                            crossterm::event::Event::Key(key)
                                if key.kind == crossterm::event::KeyEventKind::Press
                                    && tx.send(TuiEvent::Key(key)).is_err() =>
                            {
                                break;
                            }
                            crossterm::event::Event::Resize(w, h)
                                if tx.send(TuiEvent::Resize(w, h)).is_err() =>
                            {
                                break;
                            }
                            #[allow(clippy::collapsible_match)]
                            crossterm::event::Event::Paste(text) => {
                                if tx.send(TuiEvent::Paste(text)).is_err() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Send tick event for animations
                if tx.send(TuiEvent::Tick).is_err() {
                    break;
                }

                // Small delay to prevent CPU spinning
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        });
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to check if a key event matches
pub fn key_matches(event: &KeyEvent, code: KeyCode, modifiers: KeyModifiers) -> bool {
    event.code == code && event.modifiers == modifiers
}

/// Common key bindings
pub mod keys {
    use super::*;

    /// Ctrl+C - Quit
    pub fn is_quit(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('c'), KeyModifiers::CONTROL)
    }

    /// Ctrl+N - New session
    pub fn is_new_session(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('n'), KeyModifiers::CONTROL)
    }

    /// Ctrl+L - List sessions
    pub fn is_list_sessions(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('l'), KeyModifiers::CONTROL)
    }

    /// Ctrl+H - Help
    pub fn is_help(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('h'), KeyModifiers::CONTROL)
    }

    /// Ctrl+K - Clear current session
    pub fn is_clear_session(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('k'), KeyModifiers::CONTROL)
    }

    /// Ctrl+P - Toggle Plan mode
    pub fn is_toggle_plan(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('p'), KeyModifiers::CONTROL)
    }

    /// Ctrl+D - Open the Model Download dialog (Ollama)
    pub fn is_model_download(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('d'), KeyModifiers::CONTROL)
    }

    /// Ctrl+O - Open the Model Info panel
    pub fn is_model_info(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('o'), KeyModifiers::CONTROL)
    }

    /// Ctrl+W - Open the Provider Switch dialog
    pub fn is_provider_switch(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('w'), KeyModifiers::CONTROL)
    }

    /// Ctrl+G - Open the llama.cpp Local Models dialog
    pub fn is_llama_cpp_models(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('g'), KeyModifiers::CONTROL)
    }

    /// Ctrl+Y - Copy the last assistant response (or its last code block,
    /// if it has one) to the system clipboard.
    pub fn is_copy_response(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('y'), KeyModifiers::CONTROL)
    }

    /// Ctrl+V - Paste from the system clipboard at the cursor. An explicit
    /// fallback alongside bracketed paste for terminals/multiplexers where
    /// bracketed paste is unreliable.
    pub fn is_paste_clipboard(event: &KeyEvent) -> bool {
        key_matches(event, KeyCode::Char('v'), KeyModifiers::CONTROL)
    }

    /// Shift+Tab - Cycle Auto Mode (Interactive -> AutoPlan -> FullAuto ->
    /// Interactive). Most terminals report Shift+Tab as the distinct
    /// `KeyCode::BackTab`, not `Tab` with a `SHIFT` modifier bit, so match
    /// on that rather than `key_matches`.
    pub fn is_toggle_auto_mode(event: &KeyEvent) -> bool {
        event.code == KeyCode::BackTab
    }

    /// Enter - Submit (plain Enter sends; Ctrl+Enter, with or without extra
    /// modifiers, is kept as a legacy alias for muscle memory). Shift+Enter
    /// and Alt+Enter are newlines, not submit - see `is_newline`.
    pub fn is_submit(event: &KeyEvent) -> bool {
        event.code == KeyCode::Enter
            && (event.modifiers.is_empty() || event.modifiers.contains(KeyModifiers::CONTROL))
    }

    /// Shift+Enter or Alt+Enter - insert a newline in the message input.
    /// Excludes chords that also hold Ctrl (e.g. Ctrl+Shift+Enter), which
    /// `is_submit` claims instead, so the two never disagree. Shift+Enter
    /// only disambiguates from plain Enter on terminals with the Kitty
    /// keyboard protocol enabled; Alt+Enter works everywhere as the
    /// reliable fallback (see `App::kitty_keyboard_protocol_active`).
    pub fn is_newline(event: &KeyEvent) -> bool {
        event.code == KeyCode::Enter
            && !event.modifiers.contains(KeyModifiers::CONTROL)
            && (event.modifiers.contains(KeyModifiers::SHIFT)
                || event.modifiers.contains(KeyModifiers::ALT))
    }

    /// Escape - Cancel/Back
    pub fn is_cancel(event: &KeyEvent) -> bool {
        event.code == KeyCode::Esc
    }

    /// Enter - Select/Confirm
    pub fn is_enter(event: &KeyEvent) -> bool {
        event.code == KeyCode::Enter && event.modifiers.is_empty()
    }

    /// Up arrow
    pub fn is_up(event: &KeyEvent) -> bool {
        event.code == KeyCode::Up && event.modifiers.is_empty()
    }

    /// Down arrow
    pub fn is_down(event: &KeyEvent) -> bool {
        event.code == KeyCode::Down && event.modifiers.is_empty()
    }

    /// Page up
    pub fn is_page_up(event: &KeyEvent) -> bool {
        event.code == KeyCode::PageUp
    }

    /// Page down
    pub fn is_page_down(event: &KeyEvent) -> bool {
        event.code == KeyCode::PageDown
    }

    /// 'A' or 'Y' - Approve
    pub fn is_approve(event: &KeyEvent) -> bool {
        matches!(
            event.code,
            KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('y') | KeyCode::Char('Y')
        ) && event.modifiers.is_empty()
    }

    /// 'D' or 'N' - Deny
    pub fn is_deny(event: &KeyEvent) -> bool {
        matches!(
            event.code,
            KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Char('n') | KeyCode::Char('N')
        ) && event.modifiers.is_empty()
    }

    /// 'V' - View details
    pub fn is_view_details(event: &KeyEvent) -> bool {
        matches!(event.code, KeyCode::Char('v') | KeyCode::Char('V')) && event.modifiers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_handler_creation() {
        let handler = EventHandler::new();
        let sender = handler.sender();
        // Should be able to send events
        assert!(sender.send(TuiEvent::Quit).is_ok());
    }

    #[test]
    fn test_key_matches() {
        let event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(key_matches(
            &event,
            KeyCode::Char('c'),
            KeyModifiers::CONTROL
        ));
        assert!(!key_matches(
            &event,
            KeyCode::Char('c'),
            KeyModifiers::empty()
        ));
    }

    #[test]
    fn test_quit_key() {
        let event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert!(keys::is_quit(&event));

        let event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::empty());
        assert!(!keys::is_quit(&event));
    }

    #[test]
    fn test_submit_key() {
        // Plain Enter sends the message.
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        assert!(keys::is_submit(&event));

        // Ctrl+Enter is kept as a legacy alias for muscle memory.
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL);
        assert!(keys::is_submit(&event));

        // Shift+Enter and Alt+Enter are newlines, not submit.
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT);
        assert!(!keys::is_submit(&event));

        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT);
        assert!(!keys::is_submit(&event));

        // Ctrl+Shift+Enter and Ctrl+Alt+Enter still submit (Ctrl wins over
        // an incidental extra Shift/Alt bit) rather than silently becoming
        // a newline.
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL | KeyModifiers::SHIFT);
        assert!(keys::is_submit(&event));

        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL | KeyModifiers::ALT);
        assert!(keys::is_submit(&event));

        // Not an Enter key at all.
        let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        assert!(!keys::is_submit(&event));
    }

    #[test]
    fn test_model_info_key() {
        let event = KeyEvent::new(KeyCode::Char('o'), KeyModifiers::CONTROL);
        assert!(keys::is_model_info(&event));

        let event = KeyEvent::new(KeyCode::Char('o'), KeyModifiers::empty());
        assert!(!keys::is_model_info(&event));
    }

    #[test]
    fn test_provider_switch_key() {
        let event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL);
        assert!(keys::is_provider_switch(&event));

        let event = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::empty());
        assert!(!keys::is_provider_switch(&event));
    }

    #[test]
    fn test_llama_cpp_models_key() {
        let event = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::CONTROL);
        assert!(keys::is_llama_cpp_models(&event));

        let event = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::empty());
        assert!(!keys::is_llama_cpp_models(&event));
    }

    #[test]
    fn test_copy_response_key() {
        let event = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CONTROL);
        assert!(keys::is_copy_response(&event));

        let event = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::empty());
        assert!(!keys::is_copy_response(&event));
    }

    #[test]
    fn test_paste_clipboard_key() {
        let event = KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CONTROL);
        assert!(keys::is_paste_clipboard(&event));

        let event = KeyEvent::new(KeyCode::Char('v'), KeyModifiers::empty());
        assert!(!keys::is_paste_clipboard(&event));
    }

    #[test]
    fn test_toggle_auto_mode_key() {
        let event = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
        assert!(keys::is_toggle_auto_mode(&event));

        let event = KeyEvent::new(KeyCode::Tab, KeyModifiers::empty());
        assert!(!keys::is_toggle_auto_mode(&event));
    }

    #[test]
    fn test_newline_key() {
        // Shift+Enter and Alt+Enter insert a newline.
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT);
        assert!(keys::is_newline(&event));

        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT);
        assert!(keys::is_newline(&event));

        // Plain Enter and Ctrl+Enter are submit, not newline.
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        assert!(!keys::is_newline(&event));

        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL);
        assert!(!keys::is_newline(&event));

        // Ctrl+Shift+Enter and Ctrl+Alt+Enter are submit (via is_submit),
        // not newline - Ctrl must win over an incidental extra Shift/Alt
        // bit so the two predicates never both match the same event.
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL | KeyModifiers::SHIFT);
        assert!(!keys::is_newline(&event));

        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL | KeyModifiers::ALT);
        assert!(!keys::is_newline(&event));
    }
}
