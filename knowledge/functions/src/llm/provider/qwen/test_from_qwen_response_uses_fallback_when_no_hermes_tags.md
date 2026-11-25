---
type: Rust Function
title: test_from_qwen_response_uses_fallback_when_no_hermes_tags
resource: src/llm/provider/qwen.rs#L2742-L2777
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
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_from_qwen_response_uses_fallback_when_no_hermes_tags()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [from_qwen_response](../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)