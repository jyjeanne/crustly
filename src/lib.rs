//! Crustly - High-Performance Terminal AI Assistant
//!
//! A blazingly fast, memory-efficient terminal-based AI assistant for software development.
//! Written in Rust for superior performance, memory safety, and reduced resource consumption.
//!
//! ## Features
//!
//! - **Multi-LLM Support:** Anthropic, OpenAI, Google Gemini, AWS Bedrock, Azure, VertexAI
//! - **LSP Integration:** Semantic code understanding via Language Server Protocol
//! - **MCP Support:** Model Context Protocol for enhanced capabilities
//! - **Local-First:** SQLite storage for privacy and offline functionality
//! - **Modern TUI:** Built with Ratatui for responsive terminal interface
//! - **Tool System:** Extensible tools for file operations, shell commands, and more
//! - **Context Files:** Automatic loading of .cursorrules, .claudemd files
//! - **Session Management:** Persistent chat sessions with token/cost tracking
//!
//! ## Quick Start
//!
//! ```bash
//! # Interactive mode
//! crustly
//!
//! # Non-interactive mode
//! crustly run "explain this code"
//!
//! # With auto-approve (dangerous!)
//! crustly run --auto-approve "refactor this file"
//! ```

pub mod app;
pub mod cli;
pub mod config;
pub mod db;
pub mod error;
pub mod events;
pub mod llm;
pub mod lsp;
pub mod mcp;
pub mod message;
pub mod services;
pub mod sync;
pub mod tui;
pub mod utils;

// Re-export commonly used types
pub use error::{CrustlyError, ErrorCode};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
