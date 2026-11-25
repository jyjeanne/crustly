---
type: Rust Function
title: test_fallback_does_not_corrupt_unrelated_fenced_code_block
resource: src/llm/provider/qwen.rs#L2694-L2706
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_fallback_does_not_corrupt_unrelated_fenced_code_block()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [parse_fallback_tool_calls](../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)