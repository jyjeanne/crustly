//! Tool Execution Framework
//!
//! Provides an abstraction for tools that can be called by LLM agents,
//! including file operations, shell commands, and more.

pub mod error;
pub mod registry;
mod r#trait;

// Tool implementations - Phase 1: Essential File Operations
pub mod bash;
pub mod read;
pub mod write;
pub mod edit;
pub mod ls;
pub mod glob;
pub mod grep;

// Tool implementations - Phase 2: Advanced Features
pub mod web_search;
pub mod code_exec;
pub mod notebook;

// Tool implementations - Phase 3: Workflow & Integration
pub mod task;
pub mod context;
pub mod http;

// Re-exports
pub use error::{Result, ToolError};
pub use r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
pub use registry::ToolRegistry;
