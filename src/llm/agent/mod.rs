//! Agent Service Module
//!
//! Provides high-level agent functionality for managing conversations,
//! executing tools, and coordinating with LLM providers.

pub mod service;
pub mod error;
pub mod context;

// Re-exports
pub use service::{AgentService, AgentResponse, AgentStreamResponse, ToolApprovalInfo, ApprovalCallback};
pub use error::{AgentError, Result};
pub use context::AgentContext;
