---
type: Rust Method
title: send_request
resource: src/mcp/client.rs#L187-L250
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/mcp/client/MCPClient/read_response_line
  - functions/src/mcp/client/match_response_line
  called_by:
  - functions/src/mcp/client/MCPClient/connect
  - functions/src/mcp/client/MCPClient/discover_tools
  - functions/src/mcp/client/MCPClient/call_tool
  - functions/src/mcp/client/send_request_skips_a_notification_and_matches_the_response_for_its_own_id
  - functions/src/mcp/client/send_request_errors_when_the_server_process_is_gone
---

# Signature

`async fn send_request(&mut self, method: &str, params: Option<Value>) -> Result<Value>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [read_response_line](../../../../../functions/src/mcp/client/MCPClient/read_response_line.md)
- [match_response_line](../../../../../functions/src/mcp/client/match_response_line.md)

# Called by

- [connect](../../../../../functions/src/mcp/client/MCPClient/connect.md)
- [discover_tools](../../../../../functions/src/mcp/client/MCPClient/discover_tools.md)
- [call_tool](../../../../../functions/src/mcp/client/MCPClient/call_tool.md)
- [send_request_skips_a_notification_and_matches_the_response_for_its_own_id](../../../../../functions/src/mcp/client/send_request_skips_a_notification_and_matches_the_response_for_its_own_id.md)
- [send_request_errors_when_the_server_process_is_gone](../../../../../functions/src/mcp/client/send_request_errors_when_the_server_process_is_gone.md)