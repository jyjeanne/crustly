---
type: Rust Method
title: expand_span_over_adjacent_fences
resource: src/llm/provider/qwen.rs#L412-L425
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
---

# Signature

`fn expand_span_over_adjacent_fences(text: &str, start: usize, end: usize) -> (usize, usize)`

# Calls

- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [parse_fallback_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)