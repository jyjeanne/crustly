---
type: Rust Function
title: test_thinking_extraction
resource: src/llm/provider/qwen.rs#L2324-L2338
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

`fn test_thinking_extraction()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_thinking](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking.md)
- [extract_thinking](../../../../../functions/src/llm/provider/qwen/QwenProvider/extract_thinking.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)