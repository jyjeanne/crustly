---
type: Rust Function
title: connect_to_nonexistent_server_returns_error
resource: tests/mcp_contract_test.rs#L10-L24
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/mcp/client/MCPClient/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn connect_to_nonexistent_server_returns_error()`

# Calls

- [connect](../../../functions/src/mcp/client/MCPClient/connect.md)