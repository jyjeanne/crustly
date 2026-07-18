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

/// Outcome of checking one raw line read from the server against the
/// request `send_request` is waiting on.
#[derive(Debug, PartialEq)]
enum ResponseMatch {
    /// This line is the answer: a successful result.
    Result(Value),
    /// This line is the answer: the server reported an error.
    Error(Value),
    /// Not our answer - unparseable, an id-less notification, or a
    /// different in-flight request's response. Keep reading.
    Skip,
}

/// Pure decision logic for `send_request`'s read loop: does `line` answer
/// `expected_id`, and if so, how? Split out from the async read loop so the
/// id-validation/notification-skipping logic is unit-testable without
/// spawning a process.
fn match_response_line(line: &str, expected_id: u64) -> ResponseMatch {
    let response: JsonRpcResponse = match serde_json::from_str(line.trim()) {
        Ok(r) => r,
        Err(_) => return ResponseMatch::Skip,
    };
    if response.id != Some(expected_id) {
        return ResponseMatch::Skip;
    }
    match response.error {
        Some(err) => ResponseMatch::Error(err),
        None => ResponseMatch::Result(response.result.unwrap_or(Value::Null)),
    }
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
            // Without this, a server marked unhealthy (or a dropped client/
            // registry) left the spawned process running as an orphan -
            // nothing ever called `.kill()` on it.
            .kill_on_drop(true)
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
        use tokio::io::AsyncWriteExt;

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

        // A malicious/misbehaving server can send an id-less notification,
        // answer requests out of order, or never respond at all. The client
        // is shared behind a single connection with no pipelining, so
        // blindly trusting "whatever line comes back next" risks handing
        // this call's result to a *different*, unrelated call - or hanging
        // it forever. Skip anything that isn't a response to this exact
        // `id`, and bound both how long we wait per line and how many
        // stray lines we'll skip before giving up.
        const READ_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
        const MAX_SKIPPED_LINES: usize = 64;

        for _ in 0..=MAX_SKIPPED_LINES {
            let response_line = tokio::time::timeout(READ_TIMEOUT, self.read_response_line())
                .await
                .with_context(|| {
                    format!(
                        "timed out waiting for a response from MCP server '{}'",
                        self.server_name
                    )
                })??;

            match match_response_line(&response_line, id) {
                ResponseMatch::Result(value) => return Ok(value),
                ResponseMatch::Error(err) => {
                    anyhow::bail!("MCP error from '{}': {}", self.server_name, err);
                }
                ResponseMatch::Skip => {
                    tracing::debug!(
                        "Skipping unrelated/unparseable line from MCP server '{}'",
                        self.server_name
                    );
                    continue;
                }
            }
        }

        anyhow::bail!(
            "MCP server '{}' did not send a matching response for request {} after skipping {} unrelated lines",
            self.server_name,
            id,
            MAX_SKIPPED_LINES
        );
    }

    /// Read one line from the server's stdout, bounded in size so a server
    /// that never sends `\n` can't grow the buffer without limit.
    async fn read_response_line(&mut self) -> Result<String> {
        use tokio::io::AsyncReadExt;

        const MAX_LINE_BYTES: usize = 10 * 1024 * 1024;

        let mut buf = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            if buf.len() >= MAX_LINE_BYTES {
                anyhow::bail!(
                    "response line from MCP server '{}' exceeded {} bytes",
                    self.server_name,
                    MAX_LINE_BYTES
                );
            }
            let n = self.stdout.read(&mut byte).await.with_context(|| {
                format!("failed to read from MCP server '{}'", self.server_name)
            })?;
            if n == 0 {
                anyhow::bail!("MCP server '{}' closed the connection", self.server_name);
            }
            if byte[0] == b'\n' {
                break;
            }
            buf.push(byte[0]);
        }
        Ok(String::from_utf8_lossy(&buf).into_owned())
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

    /// MCP tools always require approval, regardless of the (empty)
    /// `capabilities()` above.
    ///
    /// `Tool::requires_approval()`'s default implementation only returns
    /// `true` when `capabilities()` contains a dangerous flag
    /// (`WriteFiles`/`ExecuteShell`/`SystemModification`) - and since MCP
    /// tools always report zero capabilities (the MCP protocol doesn't give
    /// Crustly a way to know what a server-defined tool actually does),
    /// that default silently resolved to `false` for every MCP tool from
    /// every configured server. Any tool an MCP server exposed - shell
    /// exec, arbitrary file write, network exfiltration - ran immediately
    /// with no approval prompt, while the equivalent built-in tool
    /// (`bash`, `write_file`) was correctly gated. Since capabilities are
    /// unknowable for an external server, the only safe default is to
    /// always require approval, the same as any other tool this crate
    /// cannot vouch for.
    fn requires_approval(&self) -> bool {
        true
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

#[cfg(test)]
mod response_matching_tests {
    use super::*;

    /// Regression: `send_request` used to blindly trust "whatever line
    /// comes back next" as the answer to its own request, with no `id`
    /// check - a desynced or out-of-order server response could silently
    /// be handed to the wrong caller.
    #[test]
    fn matches_a_response_with_the_expected_id() {
        let line = r#"{"jsonrpc":"2.0","id":5,"result":{"ok":true}}"#;
        assert_eq!(
            match_response_line(line, 5),
            ResponseMatch::Result(serde_json::json!({"ok": true}))
        );
    }

    #[test]
    fn skips_a_response_for_a_different_request_id() {
        let line = r#"{"jsonrpc":"2.0","id":6,"result":{}}"#;
        assert_eq!(match_response_line(line, 5), ResponseMatch::Skip);
    }

    #[test]
    fn skips_an_id_less_notification() {
        let line = r#"{"jsonrpc":"2.0","method":"notifications/progress","params":{}}"#;
        assert_eq!(match_response_line(line, 5), ResponseMatch::Skip);
    }

    #[test]
    fn skips_an_unparseable_line() {
        assert_eq!(
            match_response_line("not json at all", 5),
            ResponseMatch::Skip
        );
    }

    #[test]
    fn surfaces_a_server_error_for_the_matching_id() {
        let line = r#"{"jsonrpc":"2.0","id":5,"error":{"code":-1,"message":"boom"}}"#;
        assert_eq!(
            match_response_line(line, 5),
            ResponseMatch::Error(serde_json::json!({"code": -1, "message": "boom"}))
        );
    }

    #[test]
    fn missing_result_defaults_to_null() {
        let line = r#"{"jsonrpc":"2.0","id":5}"#;
        assert_eq!(
            match_response_line(line, 5),
            ResponseMatch::Result(Value::Null)
        );
    }
}

#[cfg(test)]
mod mcp_tool_approval_tests {
    use super::*;

    /// Regression: `requires_approval()` used to fall through to the trait
    /// default, which only returns `true` when `capabilities()` reports a
    /// dangerous flag - and MCP tools always report zero capabilities (the
    /// MCP protocol gives Crustly no way to know what a server-defined tool
    /// actually does), so every MCP-sourced tool from every configured
    /// server ran with no approval prompt at all.
    #[tokio::test]
    #[cfg(not(target_os = "windows"))]
    async fn mcp_tool_always_requires_approval_regardless_of_empty_capabilities() {
        // `cat` is only spawned to obtain real ChildStdin/ChildStdout
        // handles for the struct literal below - no MCP handshake is
        // performed and none of its stdio is ever used.
        let mut child = tokio::process::Command::new("cat")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("spawn cat for test fixture");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = tokio::io::BufReader::new(child.stdout.take().expect("stdout"));

        let client = MCPClient {
            server_name: "test-server".to_string(),
            stdin,
            stdout,
            _child: child,
            next_id: 1,
            healthy: true,
        };

        let tool = McpTool::new(
            "test-server",
            McpToolDef {
                name: "some_tool".to_string(),
                description: None,
                input_schema: None,
            },
            Arc::new(Mutex::new(client)),
        );

        assert!(
            tool.capabilities().is_empty(),
            "this test assumes MCP tools report no capabilities"
        );
        assert!(
            tool.requires_approval(),
            "an MCP tool with empty capabilities must still require approval"
        );
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
        assert!(
            !name.contains(':'),
            "tool name must not contain ':': {name}"
        );
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
