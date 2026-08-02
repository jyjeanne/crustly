---
type: Rust Method
title: generate_call_id
resource: src/llm/provider/qwen.rs#L232-L237
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls
  - functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
  - functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls
---

# Signature

`fn generate_call_id() -> String`

# Called by

- [parse_hermes_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls.md)
- [parse_fallback_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)
- [parse_native_qwen_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls.md)