---
type: Rust Function
title: preview_input
resource: src/llm/tools/registry.rs#L25-L32
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/registry/preview_input_truncates_a_large_payload
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/preview_input_truncates_on_char_boundaries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/preview_input_shows_the_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn preview_input(input: &Value) -> String`

# Called by

- [preview_input_truncates_a_large_payload](../../../../../functions/src/llm/tools/registry/preview_input_truncates_a_large_payload.md)
- [preview_input_truncates_on_char_boundaries](../../../../../functions/src/llm/tools/registry/preview_input_truncates_on_char_boundaries.md)
- [execute](../../../../../functions/src/llm/tools/registry/ToolRegistry/execute.md)
- [preview_input_shows_the_command](../../../../../functions/src/llm/tools/registry/preview_input_shows_the_command.md)