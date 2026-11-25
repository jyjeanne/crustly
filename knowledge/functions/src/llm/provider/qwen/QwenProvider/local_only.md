---
type: Rust Method
title: local_only
resource: src/llm/provider/qwen.rs#L912-L918
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn local_only<T>(is_local: bool, value: Option<T>) -> Option<T>`

# Called by

- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)