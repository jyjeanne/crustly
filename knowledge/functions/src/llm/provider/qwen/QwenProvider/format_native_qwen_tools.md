---
type: Rust Method
title: format_native_qwen_tools
resource: src/llm/provider/qwen.rs#L503-L534
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
  - functions/src/llm/provider/qwen/test_native_qwen_tools_format
---

# Signature

`fn format_native_qwen_tools(&self, tools: &[Tool]) -> String`

# Called by

- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [test_native_qwen_tools_format](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_tools_format.md)