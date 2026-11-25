---
type: Rust Function
title: leading_window
resource: src/llm/provider/tool_call_recovery.rs#L98-L104
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/leading_window_never_panics_and_stays_within_a_char_boundary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn leading_window(text: &str, max_bytes: usize) -> &str`

# Called by

- [commits_to_an_offered_tool_call](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call.md)
- [leading_window_never_panics_and_stays_within_a_char_boundary](../../../../../functions/src/llm/provider/tool_call_recovery/leading_window_never_panics_and_stays_within_a_char_boundary.md)