---
type: Rust Function
title: find_after
resource: src/llm/provider/qwen.rs#L105-L107
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls
  - functions/src/llm/provider/qwen/QwenProvider/extract_thinking
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
  - functions/src/llm/provider/qwen/find_after_returns_an_absolute_offset_not_a_relative_one
---

# Signature

`fn find_after(haystack: &str, start: usize, needle: &str) -> Option<usize>`

# Called by

- [parse_hermes_tool_calls](../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls.md)
- [extract_thinking](../../../../../functions/src/llm/provider/qwen/QwenProvider/extract_thinking.md)
- [from_qwen_response](../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [find_after_returns_an_absolute_offset_not_a_relative_one](../../../../../functions/src/llm/provider/qwen/find_after_returns_an_absolute_offset_not_a_relative_one.md)