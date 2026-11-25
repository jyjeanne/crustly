---
type: Rust Method
title: call_tool
resource: src/mcp/client.rs#L139-L174
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/mcp/client/MCPClient/send_request
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/client/McpTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/mcp_contract_test/unhealthy_client_returns_graceful_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String>`

# Calls

- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)

# Called by

- [execute](../../../../../functions/src/mcp/client/McpTool/tool/execute.md)
- [unhealthy_client_returns_graceful_error](../../../../../functions/tests/mcp_contract_test/unhealthy_client_returns_graceful_error.md)