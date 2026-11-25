---
type: Rust Method
title: send_request
resource: src/mcp/client.rs#L189-L252
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/read_response_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/match_response_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/mcp/client/MCPClient/connect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/discover_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/call_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/send_request_skips_a_notification_and_matches_the_response_for_its_own_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/send_request_errors_when_the_server_process_is_gone
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn send_request(&mut self, method: &str, params: Option<Value>) -> Result<Value>`

# Calls

- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [read_response_line](../../../../../functions/src/mcp/client/MCPClient/read_response_line.md)
- [match_response_line](../../../../../functions/src/mcp/client/match_response_line.md)

# Called by

- [connect](../../../../../functions/src/mcp/client/MCPClient/connect.md)
- [discover_tools](../../../../../functions/src/mcp/client/MCPClient/discover_tools.md)
- [call_tool](../../../../../functions/src/mcp/client/MCPClient/call_tool.md)
- [send_request_skips_a_notification_and_matches_the_response_for_its_own_id](../../../../../functions/src/mcp/client/send_request_skips_a_notification_and_matches_the_response_for_its_own_id.md)
- [send_request_errors_when_the_server_process_is_gone](../../../../../functions/src/mcp/client/send_request_errors_when_the_server_process_is_gone.md)