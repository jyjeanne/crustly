//! TUI Event System
//!
//! Handles user input and application events for the terminal interface.

use crate::llm::agent::AgentResponse;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Events that can occur in the TUI
#[derive(Debug, Clone)]
pub enum TuiEvent {
    /// User pressed a key
    Key(KeyEvent),

    /// Terminal was resized
    Resize(u16, u16),

    /// User submitted a message
    MessageSubmitted(String),

    /// Agent started processing
    AgentProcessing,

    /// Agent sent a response chunk (streaming)
    ResponseChunk(String),

    /// Agent completed response
    ResponseComplete(AgentResponse),

    /// An error occurred
    Error(String),

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
}

/// Application mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// Main chat interface
    Chat,
    /// Session list/management
    Sessions,
    /// Help screen
    Help,
    /// Settings
    Settings,
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
                            crossterm::event::Event::Key(key) => {
                                if tx.send(TuiEvent::Key(key)).is_err() {
                                    break;
                                }
                            }
                            crossterm::event::Event::Resize(w, h) => {
                                if tx.send(TuiEvent::Resize(w, h)).is_err() {
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

    /// Ctrl+Enter - Submit
    pub fn is_submit(event: &KeyEvent) -> bool {
        event.code == KeyCode::Enter && event.modifiers.contains(KeyModifiers::CONTROL)
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
        assert!(key_matches(&event, KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert!(!key_matches(&event, KeyCode::Char('c'), KeyModifiers::empty()));
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
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL);
        assert!(keys::is_submit(&event));

        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        assert!(!keys::is_submit(&event));
    }
}
