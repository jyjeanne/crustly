---
type: Rust Method
title: format_hermes_tools
resource: src/llm/provider/qwen.rs#L270-L291
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
  - functions/src/llm/provider/qwen/test_hermes_tools_format
---

# Signature

`fn format_hermes_tools(&self, tools: &[Tool]) -> String`

# Called by

- [to_qwen_request](../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [test_hermes_tools_format](../../../../../../functions/src/llm/provider/qwen/test_hermes_tools_format.md)