---
type: Rust Function
title: unhealthy_client_returns_graceful_error
resource: tests/mcp_contract_test.rs#L29-L54
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/mcp/client/MCPClient/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/mcp/client/MCPClient/call_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn unhealthy_client_returns_graceful_error()`

# Calls

- [connect](../../../functions/src/mcp/client/MCPClient/connect.md)
- [call_tool](../../../functions/src/mcp/client/MCPClient/call_tool.md)