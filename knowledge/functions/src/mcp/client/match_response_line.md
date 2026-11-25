---
type: Rust Function
title: match_response_line
resource: src/mcp/client.rs#L48-L60
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/mcp/client/MCPClient/send_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/matches_a_response_with_the_expected_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/skips_a_response_for_a_different_request_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/skips_an_id_less_notification
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/skips_an_unparseable_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/surfaces_a_server_error_for_the_matching_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/missing_result_defaults_to_null
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn match_response_line(line: &str, expected_id: u64) -> ResponseMatch`

# Called by

- [send_request](../../../../functions/src/mcp/client/MCPClient/send_request.md)
- [matches_a_response_with_the_expected_id](../../../../functions/src/mcp/client/matches_a_response_with_the_expected_id.md)
- [skips_a_response_for_a_different_request_id](../../../../functions/src/mcp/client/skips_a_response_for_a_different_request_id.md)
- [skips_an_id_less_notification](../../../../functions/src/mcp/client/skips_an_id_less_notification.md)
- [skips_an_unparseable_line](../../../../functions/src/mcp/client/skips_an_unparseable_line.md)
- [surfaces_a_server_error_for_the_matching_id](../../../../functions/src/mcp/client/surfaces_a_server_error_for_the_matching_id.md)
- [missing_result_defaults_to_null](../../../../functions/src/mcp/client/missing_result_defaults_to_null.md)