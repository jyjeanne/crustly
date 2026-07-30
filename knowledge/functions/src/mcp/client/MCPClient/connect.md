---
type: Rust Method
title: connect
resource: src/mcp/client.rs#L85-L126
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/mcp/client/MCPClient/send_request
---

# Signature

`pub async fn connect(server_name: &str, command: &str, args: &[&str]) -> Result<Self>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)