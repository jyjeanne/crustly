---
type: Rust Function
title: test_supported_models
resource: src/llm/provider/qwen.rs#L2359-L2367
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
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_supported_models()`

# Calls

- [dashscope_intl](../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_intl.md)
- [supported_models](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/supported_models.md)