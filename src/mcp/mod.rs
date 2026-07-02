//! Model Context Protocol Support
//!
//! Connects to external MCP tool servers over stdio (see `client`), wraps
//! their tools as `Tool` trait objects, and registers them into the
//! `ToolRegistry` at startup (`ToolRegistry::register_mcp_server`, called
//! from `cli::cmd_chat` for each `[[mcp.servers]]` entry in config).
//!
//! ## Not yet implemented
//! - Resource providers, prompt templates (MCP protocol features beyond
//!   tool discovery/invocation)
//! - Alternate transports beyond stdio (`transport` module is a stub)

pub mod client;
pub mod transport;

/// One configured MCP server's connection status, snapshotted at startup
/// for the TUI's `/mcp` view. Not live-refreshed while the view is open -
/// see the "Open Decisions" note in `ergonomy-improvment.md` about
/// deferring live reconnect-on-open as a separate enhancement.
#[derive(Debug, Clone)]
pub struct McpServerStatus {
    pub name: String,
    pub command: String,
    pub connected: bool,
    pub tool_count: usize,
    pub error: Option<String>,
}
