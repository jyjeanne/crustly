---
type: Rust Method
title: parse_native_qwen_tool_calls
resource: src/llm/provider/qwen.rs#L537-L593
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/PlanTask/skip
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/qwen/QwenProvider/generate_call_id
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
  - functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing
  - functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls
---

# Signature

`fn parse_native_qwen_tool_calls(&self, text: &str) -> Vec<(String, String, serde_json::Value)>`

# Calls

- [skip](../../../../../../functions/src/plan/PlanTask/skip.md)
- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [generate_call_id](../../../../../../functions/src/llm/provider/qwen/QwenProvider/generate_call_id.md)

# Called by

- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [test_native_qwen_tool_call_parsing](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing.md)
- [test_multiple_native_qwen_tool_calls](../../../../../../functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls.md)