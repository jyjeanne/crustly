---
type: Rust Function
title: find_after
resource: src/llm/provider/qwen.rs#L105-L107
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/extract_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after_returns_an_absolute_offset_not_a_relative_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after_ignores_a_match_before_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after_returns_none_when_nothing_matches_after_start
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_after(haystack: &str, start: usize, needle: &str) -> Option<usize>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [parse_hermes_tool_calls](../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls.md)
- [extract_thinking](../../../../../functions/src/llm/provider/qwen/QwenProvider/extract_thinking.md)
- [from_qwen_response](../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [find_after_returns_an_absolute_offset_not_a_relative_one](../../../../../functions/src/llm/provider/qwen/find_after_returns_an_absolute_offset_not_a_relative_one.md)
- [find_after_ignores_a_match_before_start](../../../../../functions/src/llm/provider/qwen/find_after_ignores_a_match_before_start.md)
- [find_after_returns_none_when_nothing_matches_after_start](../../../../../functions/src/llm/provider/qwen/find_after_returns_none_when_nothing_matches_after_start.md)