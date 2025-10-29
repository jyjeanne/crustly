//! Terminal User Interface
//!
//! Provides an interactive terminal interface for the AI assistant using Ratatui.

pub mod app;
pub mod error;
pub mod events;
pub mod render;
pub mod runner;

// Enhanced rendering modules
pub mod markdown;
pub mod highlight;
pub mod splash;

// Component modules (to be implemented)
pub mod components;
pub mod pages;
pub mod styles;
pub mod utils;

// Re-exports
pub use app::{App, DisplayMessage};
pub use events::{AppMode, EventHandler, TuiEvent};
pub use runner::run;
