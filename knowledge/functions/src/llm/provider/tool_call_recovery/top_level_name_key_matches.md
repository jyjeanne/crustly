---
type: Rust Function
title: top_level_name_key_matches
resource: src/llm/provider/tool_call_recovery.rs#L110-L116
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/tool_call_recovery/name_key_immediately_precedes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/brace_depth_at
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn top_level_name_key_matches(window: &str, tool_name: &str) -> bool`

# Calls

- [name_key_immediately_precedes](../../../../../functions/src/llm/provider/tool_call_recovery/name_key_immediately_precedes.md)
- [brace_depth_at](../../../../../functions/src/llm/provider/tool_call_recovery/brace_depth_at.md)

# Called by

- [commits_to_an_offered_tool_call](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call.md)