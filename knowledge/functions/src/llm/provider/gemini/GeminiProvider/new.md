---
type: Rust Method
title: new
resource: src/llm/provider/gemini.rs#L47-L62
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/factory/try_create_gemini
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiProvider/with_base_url
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_gemini_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_custom_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_supported_models_include_gemma
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_gemma_cost_is_free
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_calculate_cost_gemini_flash
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_tool_result_uses_function_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_keeps_text_sent_with_tool_results
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_from_gemini_response_maps_tool_use
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_from_gemini_response_maps_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_thinking_config_forwarded
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_inline_image_becomes_inline_data_part
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_image_url_source_is_skipped_without_panicking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_context_window_all_known_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_calculate_cost_all_known_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(api_key: String) -> Self`

# Calls

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)

# Called by

- [try_create_gemini](../../../../../../functions/src/llm/provider/factory/try_create_gemini.md)
- [with_base_url](../../../../../../functions/src/llm/provider/gemini/GeminiProvider/with_base_url.md)
- [test_gemini_provider_creation](../../../../../../functions/src/llm/provider/gemini/test_gemini_provider_creation.md)
- [test_custom_default_model](../../../../../../functions/src/llm/provider/gemini/test_custom_default_model.md)
- [test_supported_models_include_gemma](../../../../../../functions/src/llm/provider/gemini/test_supported_models_include_gemma.md)
- [test_context_window](../../../../../../functions/src/llm/provider/gemini/test_context_window.md)
- [test_gemma_cost_is_free](../../../../../../functions/src/llm/provider/gemini/test_gemma_cost_is_free.md)
- [test_calculate_cost_gemini_flash](../../../../../../functions/src/llm/provider/gemini/test_calculate_cost_gemini_flash.md)
- [test_to_gemini_request_maps_system_and_tools](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools.md)
- [test_to_gemini_request_tool_result_uses_function_name](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_tool_result_uses_function_name.md)
- [test_to_gemini_request_keeps_text_sent_with_tool_results](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_keeps_text_sent_with_tool_results.md)
- [test_from_gemini_response_maps_tool_use](../../../../../../functions/src/llm/provider/gemini/test_from_gemini_response_maps_tool_use.md)
- [test_from_gemini_response_maps_thinking](../../../../../../functions/src/llm/provider/gemini/test_from_gemini_response_maps_thinking.md)
- [test_thinking_config_forwarded](../../../../../../functions/src/llm/provider/gemini/test_thinking_config_forwarded.md)
- [test_json_mode_sets_response_mime_type](../../../../../../functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type.md)
- [test_full_json_schema_sets_response_schema](../../../../../../functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema.md)
- [test_inline_image_becomes_inline_data_part](../../../../../../functions/src/llm/provider/gemini/test_inline_image_becomes_inline_data_part.md)
- [test_image_url_source_is_skipped_without_panicking](../../../../../../functions/src/llm/provider/gemini/test_image_url_source_is_skipped_without_panicking.md)
- [test_context_window_all_known_models](../../../../../../functions/src/llm/provider/gemini/test_context_window_all_known_models.md)
- [test_calculate_cost_all_known_models](../../../../../../functions/src/llm/provider/gemini/test_calculate_cost_all_known_models.md)