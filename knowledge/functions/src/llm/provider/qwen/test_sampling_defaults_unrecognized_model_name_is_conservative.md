---
type: Rust Function
title: test_sampling_defaults_unrecognized_model_name_is_conservative
resource: src/llm/provider/qwen.rs#L2870-L2878
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

`fn test_sampling_defaults_unrecognized_model_name_is_conservative()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [to_qwen_request](../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)