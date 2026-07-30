---
type: Rust Method
title: format_native_qwen_result
resource: src/llm/provider/qwen.rs#L596-L598
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
  - functions/src/llm/provider/qwen/test_native_qwen_result_format
---

# Signature

`fn format_native_qwen_result(&self, result: &str) -> String`

# Called by

- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [test_native_qwen_result_format](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_result_format.md)