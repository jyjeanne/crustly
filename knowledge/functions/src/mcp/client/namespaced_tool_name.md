---
type: Rust Function
title: namespaced_tool_name
resource: src/mcp/client.rs#L308-L310
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/mcp/client/McpTool/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/namespaced_tool_name_contains_no_colons
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/namespaced_tool_name_matches_provider_function_name_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/namespaced_tool_name_uses_double_underscore_convention
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn namespaced_tool_name(server_name: &str, tool_name: &str) -> String`

# Called by

- [new](../../../../functions/src/mcp/client/McpTool/new.md)
- [namespaced_tool_name_contains_no_colons](../../../../functions/src/mcp/client/namespaced_tool_name_contains_no_colons.md)
- [namespaced_tool_name_matches_provider_function_name_pattern](../../../../functions/src/mcp/client/namespaced_tool_name_matches_provider_function_name_pattern.md)
- [namespaced_tool_name_uses_double_underscore_convention](../../../../functions/src/mcp/client/namespaced_tool_name_uses_double_underscore_convention.md)