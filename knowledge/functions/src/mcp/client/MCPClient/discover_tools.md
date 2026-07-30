---
type: Rust Method
title: discover_tools
resource: src/mcp/client.rs#L129-L136
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/mcp/client/MCPClient/send_request
  called_by:
  - functions/src/llm/tools/registry/ToolRegistry/register_mcp_server
---

# Signature

`pub async fn discover_tools(&mut self) -> Result<Vec<McpToolDef>>`

# Calls

- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)

# Called by

- [register_mcp_server](../../../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)