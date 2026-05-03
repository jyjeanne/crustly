//! MCP Transport Layer
//!
//! Newline-delimited JSON-RPC over stdio.
//! The transport itself is embedded in `MCPClient` in `src/mcp/client.rs`:
//! - `MCPClient::connect(name, command, args)` spawns the subprocess and pipes stdio
//! - `send_request` / `call_tool` frame messages as `{"jsonrpc":"2.0",...}\n`
//!
//! This module exists as a namespace for future alternative transports (HTTP, WebSocket).
