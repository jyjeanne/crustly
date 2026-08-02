---
type: Rust Method
title: parse_hermes_tool_calls
resource: src/llm/provider/qwen.rs#L294-L341
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/qwen/find_after
  - functions/src/llm/provider/qwen/QwenProvider/generate_call_id
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
  - functions/src/llm/provider/qwen/test_hermes_tool_call_parsing
  - functions/src/llm/provider/qwen/test_multiple_hermes_tool_calls
  - functions/src/llm/provider/qwen/test_hermes_malformed_json_is_skipped_without_panicking
  - functions/src/llm/provider/qwen/test_hermes_json_missing_required_fields_is_skipped
---

# Signature

`fn parse_hermes_tool_calls(&self, text: &str) -> Vec<(String, String, serde_json::Value)>`

# Calls

- [find_after](../../../../../../functions/src/llm/provider/qwen/find_after.md)
- [generate_call_id](../../../../../../functions/src/llm/provider/qwen/QwenProvider/generate_call_id.md)

# Called by

- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [test_hermes_tool_call_parsing](../../../../../../functions/src/llm/provider/qwen/test_hermes_tool_call_parsing.md)
- [test_multiple_hermes_tool_calls](../../../../../../functions/src/llm/provider/qwen/test_multiple_hermes_tool_calls.md)
- [test_hermes_malformed_json_is_skipped_without_panicking](../../../../../../functions/src/llm/provider/qwen/test_hermes_malformed_json_is_skipped_without_panicking.md)
- [test_hermes_json_missing_required_fields_is_skipped](../../../../../../functions/src/llm/provider/qwen/test_hermes_json_missing_required_fields_is_skipped.md)