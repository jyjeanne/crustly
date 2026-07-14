//! MCP client: JSON-RPC 2.0 over stdio transport.
//!
//! Connects to an external MCP server process, discovers its tools,
//! and wraps them as `Tool` trait objects for the `ToolRegistry`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── JSON-RPC types ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

// ── Tool definition returned by tools/list ────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolDef {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Option<Value>,
}

// ── MCPClient ─────────────────────────────────────────────────────────────────

/// A live connection to one MCP tool server process.
pub struct MCPClient {
    server_name: String,
    stdin: tokio::process::ChildStdin,
    stdout: tokio::io::BufReader<tokio::process::ChildStdout>,
    _child: tokio::process::Child,
    next_id: u64,
    healthy: bool,
}

impl MCPClient {
    /// Spawn the MCP server process and perform the initialize handshake.
    pub async fn connect(server_name: &str, command: &str, args: &[&str]) -> Result<Self> {
        use tokio::process::Command;

        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .with_context(|| format!("failed to spawn MCP server '{}'", server_name))?;

        let stdin = child.stdin.take().context("failed to get MCP stdin")?;
        let stdout =
            tokio::io::BufReader::new(child.stdout.take().context("failed to get MCP stdout")?);

        let mut client = Self {
            server_name: server_name.to_string(),
            stdin,
            stdout,
            _child: child,
            next_id: 1,
            healthy: true,
        };

        // Send initialize
        client
            .send_request(
                "initialize",
                Some(serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": { "name": "crustly", "version": env!("CARGO_PKG_VERSION") }
                })),
            )
            .await?;

        Ok(client)
    }

    /// Discover tools exposed by the server.
    pub async fn discover_tools(&mut self) -> Result<Vec<McpToolDef>> {
        let result = self.send_request("tools/list", None).await?;
        let tools = result
            .get("tools")
            .and_then(|v| serde_json::from_value::<Vec<McpToolDef>>(v.clone()).ok())
            .unwrap_or_default();
        Ok(tools)
    }

    /// Call a specific tool on the server.
    pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String> {
        if !self.healthy {
            anyhow::bail!("MCP server '{}' is unavailable", self.server_name);
        }

        let result = self
            .send_request(
                "tools/call",
                Some(serde_json::json!({
                    "name": name,
                    "arguments": arguments
                })),
            )
            .await
            .map_err(|e| {
                self.healthy = false;
                e
            })?;

        // Extract text content from the result
        if let Some(content) = result.get("content").and_then(|c| c.as_array()) {
            let text = content
                .iter()
                .filter_map(|item| {
                    if item.get("type").and_then(|t| t.as_str()) == Some("text") {
                        item.get("text").and_then(|t| t.as_str()).map(String::from)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            return Ok(text);
        }

        Ok(result.to_string())
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy
    }

    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    // ── private ──────────────────────────────────────────────────────────────

    async fn send_request(&mut self, method: &str, params: Option<Value>) -> Result<Value> {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

        let id = self.next_id;
        self.next_id += 1;

        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.to_string(),
            params,
        };

        let mut line = serde_json::to_string(&req)?;
        line.push('\n');
        self.stdin
            .write_all(line.as_bytes())
            .await
            .with_context(|| format!("failed to write to MCP server '{}'", self.server_name))?;
        self.stdin.flush().await?;

        let mut response_line = String::new();
        self.stdout
            .read_line(&mut response_line)
            .await
            .with_context(|| format!("failed to read from MCP server '{}'", self.server_name))?;

        let response: JsonRpcResponse = serde_json::from_str(response_line.trim())
            .with_context(|| format!("invalid JSON-RPC response from '{}'", self.server_name))?;

        if let Some(err) = response.error {
            anyhow::bail!("MCP error from '{}': {}", self.server_name, err);
        }

        Ok(response.result.unwrap_or(Value::Null))
    }
}

// ── McpTool wrapper ───────────────────────────────────────────────────────────

use crate::llm::tools::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Wraps an MCP tool as a `Tool` trait object.
pub struct McpTool {
    namespaced_name: String,
    def: McpToolDef,
    client: Arc<Mutex<MCPClient>>,
}

/// Build the namespaced tool name exposed to the model for an MCP tool.
///
/// `__` (not `::`) because most providers validate function-call names
/// against `^[a-zA-Z0-9_-]{1,64}$` - a `:` is rejected by OpenAI/Qwen
/// (DashScope) tool-calling and would make every MCP tool uncallable the
/// moment a server is configured. `__` also matches the namespacing
/// convention used by Claude Code and qwen-code (`mcp__{server}__{tool}`),
/// so a model trained on either guesses Crustly's MCP tool names correctly.
pub fn namespaced_tool_name(server_name: &str, tool_name: &str) -> String {
    format!("mcp__{}__{}", server_name, tool_name)
}

impl McpTool {
    pub fn new(server_name: &str, def: McpToolDef, client: Arc<Mutex<MCPClient>>) -> Self {
        Self {
            namespaced_name: namespaced_tool_name(server_name, &def.name),
            def,
            client,
        }
    }
}

#[cfg(test)]
mod mcp_tool_naming_tests {
    use super::*;

    /// Regression: a `:` in the tool name is rejected by OpenAI/Qwen
    /// function-calling name validation (`^[a-zA-Z0-9_-]{1,64}$`), which
    /// would make every MCP tool uncallable under those providers.
    #[test]
    fn namespaced_tool_name_contains_no_colons() {
        let name = namespaced_tool_name("github", "get_me");
        assert!(!name.contains(':'), "tool name must not contain ':': {name}");
    }

    /// Must match the `mcp__{server}__{tool}` convention used by Claude
    /// Code and qwen-code, so models trained on either guess correctly.
    #[test]
    fn namespaced_tool_name_uses_double_underscore_convention() {
        assert_eq!(
            namespaced_tool_name("github", "get_me"),
            "mcp__github__get_me"
        );
    }

    #[test]
    fn namespaced_tool_name_matches_provider_function_name_pattern() {
        let name = namespaced_tool_name("my-server_1", "some_tool");
        assert!(
            name.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'),
            "tool name must match ^[a-zA-Z0-9_-]+$: {name}"
        );
        assert!(name.len() <= 64, "tool name must be <=64 chars: {name}");
    }
}

#[async_trait]
impl Tool for McpTool {
    fn name(&self) -> &str {
        &self.namespaced_name
    }

    fn description(&self) -> &str {
        self.def.description.as_deref().unwrap_or("MCP tool")
    }

    fn input_schema(&self) -> Value {
        self.def
            .input_schema
            .clone()
            .unwrap_or_else(|| serde_json::json!({}))
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![] // MCP tools declare no built-in capabilities
    }

    async fn execute(
        &self,
        input: Value,
        _ctx: &ToolExecutionContext,
    ) -> crate::llm::tools::Result<ToolResult> {
        let mut client = self.client.lock().await;
        match client.call_tool(&self.def.name, input).await {
            Ok(output) => Ok(ToolResult::success(output)),
            Err(e) => Ok(ToolResult::error(e.to_string())),
        }
    }
}
