---
type: Rust Function
title: test_thinking_extraction_out_of_order_tags_does_not_panic
resource: src/llm/provider/qwen.rs#L2346-L2356
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/with_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/extract_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_thinking_extraction_out_of_order_tags_does_not_panic()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_thinking](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking.md)
- [extract_thinking](../../../../../functions/src/llm/provider/qwen/QwenProvider/extract_thinking.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)