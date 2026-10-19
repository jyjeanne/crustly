//! Agent Service Module
//!
//! Provides high-level agent functionality for managing conversations,
//! executing tools, and coordinating with LLM providers.

pub mod budget;
pub mod compaction;
pub mod context;
pub mod error;
pub mod memory;
pub mod middleware;
pub mod project_memory;
pub mod service;
pub mod tool_profile;

// Re-exports
pub use budget::{ContextBudget, ContextUsage};
pub use context::AgentContext;
pub use error::{AgentError, Result};
pub use service::{
    AgentResponse, AgentService, AgentStreamResponse, ApprovalCallback, ToolApprovalInfo,
};
