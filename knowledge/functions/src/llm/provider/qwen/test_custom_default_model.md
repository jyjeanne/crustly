---
type: Rust Function
title: test_custom_default_model
resource: src/llm/provider/qwen.rs#L2408-L2412
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
  - target: functions/src/llm/provider/qwen/QwenProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_custom_default_model()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_default_model](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_default_model.md)
- [default_model](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/default_model.md)