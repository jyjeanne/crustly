---
type: Rust Function
title: namespaced_tool_name
resource: src/mcp/client.rs#L306-L308
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/mcp/client/McpTool/new
  - functions/src/mcp/client/namespaced_tool_name_contains_no_colons
  - functions/src/mcp/client/namespaced_tool_name_matches_provider_function_name_pattern
---

# Signature

`pub fn namespaced_tool_name(server_name: &str, tool_name: &str) -> String`

# Called by

- [new](../../../../functions/src/mcp/client/McpTool/new.md)
- [namespaced_tool_name_contains_no_colons](../../../../functions/src/mcp/client/namespaced_tool_name_contains_no_colons.md)
- [namespaced_tool_name_matches_provider_function_name_pattern](../../../../functions/src/mcp/client/namespaced_tool_name_matches_provider_function_name_pattern.md)