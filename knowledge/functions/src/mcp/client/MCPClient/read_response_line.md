---
type: Rust Method
title: read_response_line
resource: src/mcp/client.rs#L256-L283
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/client/MCPClient/send_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn read_response_line(&mut self) -> Result<String>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)