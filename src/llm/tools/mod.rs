//! Tool Execution Framework
//!
//! Provides an abstraction for tools that can be called by LLM agents,
//! including file operations, shell commands, and more.

pub mod error;
pub mod registry;
mod r#trait;

// Tool implementations
pub mod bash;
pub mod read;
pub mod write;

// Re-exports
pub use error::{ToolError, Result};
pub use r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
pub use registry::ToolRegistry;
