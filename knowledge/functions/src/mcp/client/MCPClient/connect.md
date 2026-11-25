---
type: Rust Method
title: connect
resource: src/mcp/client.rs#L88-L129
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/send_request
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/registry/ToolRegistry/register_mcp_server
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/detect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/mcp_contract_test/connect_to_nonexistent_server_returns_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/mcp_contract_test/unhealthy_client_returns_graceful_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/mcp_contract_test/connect_times_out_gracefully
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn connect(server_name: &str, command: &str, args: &[&str]) -> Result<Self>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)
- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)

# Called by

- [register_mcp_server](../../../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)
- [detect](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/detect.md)
- [connect_to_nonexistent_server_returns_error](../../../../../functions/tests/mcp_contract_test/connect_to_nonexistent_server_returns_error.md)
- [unhealthy_client_returns_graceful_error](../../../../../functions/tests/mcp_contract_test/unhealthy_client_returns_graceful_error.md)
- [connect_times_out_gracefully](../../../../../functions/tests/mcp_contract_test/connect_times_out_gracefully.md)