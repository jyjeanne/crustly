---
type: Rust Method
title: default_local
resource: src/llm/provider/ollama.rs#L157-L159
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
  - functions/src/llm/provider/ollama/test_ollama_provider_creation
  - functions/src/llm/provider/ollama/test_with_default_model
  - functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model
  - functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field
  - functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map
  - functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested
  - functions/src/llm/provider/ollama/test_validate_model_always_true
  - functions/src/llm/provider/ollama/test_supported_models_includes_gemma4
  - functions/src/llm/provider/ollama/test_supported_models_includes_ornith
  - functions/src/llm/provider/ollama/test_context_window_default_and_custom
  - functions/src/llm/provider/ollama/test_calculate_cost_is_always_zero
  - functions/src/llm/provider/ollama/test_supports_vision_detection
  - functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
  - functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block
  - functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block
  - functions/src/llm/provider/ollama/from_ollama_response_plain_text_with_final_data
  - functions/src/llm/provider/ollama/from_ollama_response_without_final_data_has_zero_usage_and_no_perf
  - functions/src/llm/provider/ollama/from_ollama_response_extracts_tool_calls
  - functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller
  - functions/src/llm/provider/ollama/from_ollama_response_uses_explicit_thinking_field
  - functions/src/llm/provider/ollama/from_ollama_response_falls_back_to_think_tags
  - functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages
  - functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format
  - functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking
  - functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think
  - functions/src/llm/provider/ollama/invalid_think_value_is_ignored
  - functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image
---

# Signature

`pub fn default_local() -> Self`

# Called by

- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [test_ollama_provider_creation](../../../../../../functions/src/llm/provider/ollama/test_ollama_provider_creation.md)
- [test_with_default_model](../../../../../../functions/src/llm/provider/ollama/test_with_default_model.md)
- [per_model_override_wins_over_provider_default_for_that_model](../../../../../../functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model.md)
- [per_model_override_falls_back_field_by_field](../../../../../../functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field.md)
- [overrides_for_returns_provider_defaults_when_no_per_model_map](../../../../../../functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map.md)
- [context_window_reflects_the_per_model_num_ctx_that_is_actually_requested](../../../../../../functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested.md)
- [test_validate_model_always_true](../../../../../../functions/src/llm/provider/ollama/test_validate_model_always_true.md)
- [test_supported_models_includes_gemma4](../../../../../../functions/src/llm/provider/ollama/test_supported_models_includes_gemma4.md)
- [test_supported_models_includes_ornith](../../../../../../functions/src/llm/provider/ollama/test_supported_models_includes_ornith.md)
- [test_context_window_default_and_custom](../../../../../../functions/src/llm/provider/ollama/test_context_window_default_and_custom.md)
- [test_calculate_cost_is_always_zero](../../../../../../functions/src/llm/provider/ollama/test_calculate_cost_is_always_zero.md)
- [test_supports_vision_detection](../../../../../../functions/src/llm/provider/ollama/test_supports_vision_detection.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [recovered_tool_call_becomes_a_tool_use_block](../../../../../../functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block.md)
- [fenced_call_in_prose_becomes_a_tool_use_block](../../../../../../functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block.md)
- [from_ollama_response_plain_text_with_final_data](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_plain_text_with_final_data.md)
- [from_ollama_response_without_final_data_has_zero_usage_and_no_perf](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_without_final_data_has_zero_usage_and_no_perf.md)
- [from_ollama_response_extracts_tool_calls](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_extracts_tool_calls.md)
- [streamed_tool_call_reaches_caller](../../../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)
- [from_ollama_response_uses_explicit_thinking_field](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_uses_explicit_thinking_field.md)
- [from_ollama_response_falls_back_to_think_tags](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_falls_back_to_think_tags.md)
- [to_ollama_request_maps_tool_messages](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages.md)
- [to_ollama_request_maps_thinking_and_response_format](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format.md)
- [per_model_think_false_is_sent_when_request_has_no_thinking](../../../../../../functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking.md)
- [request_thinking_wins_over_configured_think](../../../../../../functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think.md)
- [invalid_think_value_is_ignored](../../../../../../functions/src/llm/provider/ollama/invalid_think_value_is_ignored.md)
- [to_ollama_request_embeds_base64_image](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image.md)