---
type: Rust Method
title: local_only
resource: src/llm/provider/qwen.rs#L902-L908
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
---

# Signature

`fn local_only<T>(is_local: bool, value: Option<T>) -> Option<T>`

# Called by

- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)