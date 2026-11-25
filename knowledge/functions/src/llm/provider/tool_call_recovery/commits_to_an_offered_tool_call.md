---
type: Rust Function
title: commits_to_an_offered_tool_call
resource: src/llm/provider/tool_call_recovery.rs#L71-L80
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/tool_call_recovery/leading_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/top_level_name_key_matches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_requires_a_real_tool_name_not_just_a_brace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_recognizes_compact_and_spaced_name_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_rejects_a_fenced_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_requires_leading_brace_not_just_a_substring_match
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_rejects_a_nested_name_field
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_accepts_the_top_level_name_after_other_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_does_not_scan_past_the_window
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn commits_to_an_offered_tool_call(text: &str, offered: &[Tool]) -> bool`

# Calls

- [leading_window](../../../../../functions/src/llm/provider/tool_call_recovery/leading_window.md)
- [top_level_name_key_matches](../../../../../functions/src/llm/provider/tool_call_recovery/top_level_name_key_matches.md)

# Called by

- [maybe_swap_to_constrained_sampler](../../../../../functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler.md)
- [commits_to_an_offered_tool_call_requires_a_real_tool_name_not_just_a_brace](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_requires_a_real_tool_name_not_just_a_brace.md)
- [commits_to_an_offered_tool_call_recognizes_compact_and_spaced_name_keys](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_recognizes_compact_and_spaced_name_keys.md)
- [commits_to_an_offered_tool_call_rejects_a_fenced_block](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_rejects_a_fenced_block.md)
- [commits_to_an_offered_tool_call_requires_leading_brace_not_just_a_substring_match](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_requires_leading_brace_not_just_a_substring_match.md)
- [commits_to_an_offered_tool_call_rejects_a_nested_name_field](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_rejects_a_nested_name_field.md)
- [commits_to_an_offered_tool_call_accepts_the_top_level_name_after_other_keys](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_accepts_the_top_level_name_after_other_keys.md)
- [commits_to_an_offered_tool_call_does_not_scan_past_the_window](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_does_not_scan_past_the_window.md)