---
type: Rust Function
title: test_sampling_defaults_dashscope_omits_vendor_extensions
resource: src/llm/provider/qwen.rs#L2487-L2495
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/dashscope_intl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_sampling_defaults_dashscope_omits_vendor_extensions()`

# Calls

- [dashscope_intl](../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_intl.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [to_qwen_request](../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)