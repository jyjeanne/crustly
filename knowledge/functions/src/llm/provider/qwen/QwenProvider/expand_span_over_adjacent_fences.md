---
type: Rust Method
title: expand_span_over_adjacent_fences
resource: src/llm/provider/qwen.rs#L412-L425
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn expand_span_over_adjacent_fences(text: &str, start: usize, end: usize) -> (usize, usize)`

# Called by

- [parse_fallback_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)