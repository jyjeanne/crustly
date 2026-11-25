---
type: Rust Method
title: to_gemini_request
resource: src/llm/provider/gemini.rs#L113-L267
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/gemini_role
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiPart/text
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiPart/with_inline_data
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_tool_result_uses_function_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_keeps_text_sent_with_tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_thinking_config_forwarded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_inline_image_becomes_inline_data_part
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_image_url_source_is_skipped_without_panicking
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_gemini_request(&self, request: &LLMRequest) -> GeminiRequest`

# Calls

- [gemini_role](../../../../../../functions/src/llm/provider/gemini/gemini_role.md)
- [text](../../../../../../functions/src/llm/provider/gemini/GeminiPart/text.md)
- [with_inline_data](../../../../../../functions/src/llm/provider/gemini/GeminiPart/with_inline_data.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/stream.md)
- [test_to_gemini_request_maps_system_and_tools](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools.md)
- [test_to_gemini_request_tool_result_uses_function_name](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_tool_result_uses_function_name.md)
- [test_to_gemini_request_keeps_text_sent_with_tool_results](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_keeps_text_sent_with_tool_results.md)
- [test_thinking_config_forwarded](../../../../../../functions/src/llm/provider/gemini/test_thinking_config_forwarded.md)
- [test_json_mode_sets_response_mime_type](../../../../../../functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type.md)
- [test_full_json_schema_sets_response_schema](../../../../../../functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema.md)
- [test_inline_image_becomes_inline_data_part](../../../../../../functions/src/llm/provider/gemini/test_inline_image_becomes_inline_data_part.md)
- [test_image_url_source_is_skipped_without_panicking](../../../../../../functions/src/llm/provider/gemini/test_image_url_source_is_skipped_without_panicking.md)