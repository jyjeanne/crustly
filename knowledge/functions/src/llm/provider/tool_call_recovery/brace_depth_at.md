---
type: Rust Function
title: brace_depth_at
resource: src/llm/provider/tool_call_recovery.rs#L133-L154
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/tool_call_recovery/top_level_name_key_matches
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn brace_depth_at(text: &str, byte_pos: usize) -> i32`

# Called by

- [top_level_name_key_matches](../../../../../functions/src/llm/provider/tool_call_recovery/top_level_name_key_matches.md)