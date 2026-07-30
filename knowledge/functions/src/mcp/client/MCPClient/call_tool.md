---
type: Rust Method
title: call_tool
resource: src/mcp/client.rs#L139-L175
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/mcp/client/MCPClient/send_request
  called_by:
  - functions/src/mcp/client/McpTool/tool/execute
  - functions/tests/mcp_contract_test/unhealthy_client_returns_graceful_error
---

# Signature

`pub async fn call_tool(&mut self, name: &str, arguments: Value) -> Result<String>`

# Calls

- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)

# Called by

- [execute](../../../../../functions/src/mcp/client/McpTool/tool/execute.md)
- [unhealthy_client_returns_graceful_error](../../../../../functions/tests/mcp_contract_test/unhealthy_client_returns_graceful_error.md)