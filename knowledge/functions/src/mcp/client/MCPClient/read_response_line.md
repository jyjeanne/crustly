---
type: Rust Method
title: read_response_line
resource: src/mcp/client.rs#L254-L281
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/tui/error/ErrorInfo/with_context
  called_by:
  - functions/src/mcp/client/MCPClient/send_request
---

# Signature

`async fn read_response_line(&mut self) -> Result<String>`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)