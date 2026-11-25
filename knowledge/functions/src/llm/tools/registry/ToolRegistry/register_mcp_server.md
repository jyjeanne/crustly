---
type: Rust Method
title: register_mcp_server
resource: src/llm/tools/registry.rs#L251-L278
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/mcp/client/MCPClient/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/mcp/client/MCPClient/discover_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/McpTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/mcp/client/McpTool/tool/name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/connect_configured_mcp_servers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn register_mcp_server( &mut self, server_name: &str, command: &str, args: &[&str], ) -> anyhow::Result<usize>`

# Calls

- [connect](../../../../../../functions/src/mcp/client/MCPClient/connect.md)
- [discover_tools](../../../../../../functions/src/mcp/client/MCPClient/discover_tools.md)
- [new](../../../../../../functions/src/mcp/client/McpTool/new.md)
- [name](../../../../../../functions/src/mcp/client/McpTool/tool/name.md)

# Called by

- [connect_configured_mcp_servers](../../../../../../functions/src/cli/connect_configured_mcp_servers.md)
- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [register_mcp_server_with_nonexistent_command_fails_gracefully](../../../../../../functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully.md)