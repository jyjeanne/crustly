---
type: Rust Function
title: test_text_sent_with_tool_results_follows_them
resource: src/llm/provider/qwen.rs#L2431-L2456
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
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_text_sent_with_tool_results_follows_them()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [to_qwen_request](../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)