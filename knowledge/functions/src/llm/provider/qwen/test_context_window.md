---
type: Rust Function
title: test_context_window
resource: src/llm/provider/qwen.rs#L2370-L2376
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/dashscope_intl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_context_window()`

# Calls

- [dashscope_intl](../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_intl.md)
- [context_window](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/context_window.md)