---
type: Rust Function
title: match_response_line
resource: src/mcp/client.rs#L48-L60
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  called_by:
  - functions/src/mcp/client/MCPClient/send_request
---

# Signature

`fn match_response_line(line: &str, expected_id: u64) -> ResponseMatch`

# Calls

- [from_str](../../../../functions/src/config/secrets/SecretString/from_str.md)

# Called by

- [send_request](../../../../functions/src/mcp/client/MCPClient/send_request.md)