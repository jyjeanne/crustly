---
type: Rust Module
title: tool_call_recovery
resource: src/llm/provider/tool_call_recovery.rs#L1-L493
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-types-tool
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [maybe_tool_call_json](../../../../functions/src/llm/provider/tool_call_recovery/maybe_tool_call_json.md)
- [commits_to_an_offered_tool_call](../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call.md)
- [leading_window](../../../../functions/src/llm/provider/tool_call_recovery/leading_window.md)
- [top_level_name_key_matches](../../../../functions/src/llm/provider/tool_call_recovery/top_level_name_key_matches.md)
- [name_key_immediately_precedes](../../../../functions/src/llm/provider/tool_call_recovery/name_key_immediately_precedes.md)
- [brace_depth_at](../../../../functions/src/llm/provider/tool_call_recovery/brace_depth_at.md)
- [tool_call_from_content](../../../../functions/src/llm/provider/tool_call_recovery/tool_call_from_content.md)
- [fenced_json_blocks](../../../../functions/src/llm/provider/tool_call_recovery/fenced_json_blocks.md)
- [parse_tool_call_object](../../../../functions/src/llm/provider/tool_call_recovery/parse_tool_call_object.md)
- [bash_tool](../../../../functions/src/llm/provider/tool_call_recovery/bash_tool.md)
- [tool_call_printed_as_content_is_recovered](../../../../functions/src/llm/provider/tool_call_recovery/tool_call_printed_as_content_is_recovered.md)
- [tool_call_in_a_json_fence_is_recovered](../../../../functions/src/llm/provider/tool_call_recovery/tool_call_in_a_json_fence_is_recovered.md)
- [tool_call_in_a_fence_embedded_in_prose_is_recovered](../../../../functions/src/llm/provider/tool_call_recovery/tool_call_in_a_fence_embedded_in_prose_is_recovered.md)
- [first_of_several_fenced_calls_is_recovered](../../../../functions/src/llm/provider/tool_call_recovery/first_of_several_fenced_calls_is_recovered.md)
- [fenced_non_tool_json_is_not_recovered](../../../../functions/src/llm/provider/tool_call_recovery/fenced_non_tool_json_is_not_recovered.md)
- [prose_is_never_mistaken_for_a_tool_call](../../../../functions/src/llm/provider/tool_call_recovery/prose_is_never_mistaken_for_a_tool_call.md)
- [only_json_like_content_is_withheld_from_streaming](../../../../functions/src/llm/provider/tool_call_recovery/only_json_like_content_is_withheld_from_streaming.md)
- [commits_to_an_offered_tool_call_requires_a_real_tool_name_not_just_a_brace](../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_requires_a_real_tool_name_not_just_a_brace.md)
- [commits_to_an_offered_tool_call_recognizes_compact_and_spaced_name_keys](../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_recognizes_compact_and_spaced_name_keys.md)
- [commits_to_an_offered_tool_call_rejects_a_fenced_block](../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_rejects_a_fenced_block.md)
- [commits_to_an_offered_tool_call_requires_leading_brace_not_just_a_substring_match](../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_requires_leading_brace_not_just_a_substring_match.md)
- [commits_to_an_offered_tool_call_rejects_a_nested_name_field](../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_rejects_a_nested_name_field.md)
- [commits_to_an_offered_tool_call_accepts_the_top_level_name_after_other_keys](../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_accepts_the_top_level_name_after_other_keys.md)
- [leading_window_never_panics_and_stays_within_a_char_boundary](../../../../functions/src/llm/provider/tool_call_recovery/leading_window_never_panics_and_stays_within_a_char_boundary.md)
- [commits_to_an_offered_tool_call_does_not_scan_past_the_window](../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call_does_not_scan_past_the_window.md)

# Imports

- `super::types::Tool`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)