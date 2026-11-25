---
type: Rust Method
title: discover_tools
resource: src/mcp/client.rs#L132-L139
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/mcp/client/MCPClient/send_request
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/registry/ToolRegistry/register_mcp_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/MCPClient/okfserver/tools
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn discover_tools(&mut self) -> Result<Vec<McpToolDef>>`

# Calls

- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)

# Called by

- [register_mcp_server](../../../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)
- [tools](../../../../../functions/src/memory/code_knowledge/MCPClient/okfserver/tools.md)