---
type: Rust Method
title: new
resource: src/mcp/client.rs#L310-L316
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/mcp/client/namespaced_tool_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/registry/ToolRegistry/register_mcp_server
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/mcp/client/mcp_tool_always_requires_approval_regardless_of_empty_capabilities
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(server_name: &str, def: McpToolDef, client: Arc<Mutex<MCPClient>>) -> Self`

# Calls

- [namespaced_tool_name](../../../../../functions/src/mcp/client/namespaced_tool_name.md)

# Called by

- [register_mcp_server](../../../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)
- [mcp_tool_always_requires_approval_regardless_of_empty_capabilities](../../../../../functions/src/mcp/client/mcp_tool_always_requires_approval_regardless_of_empty_capabilities.md)