---
type: Rust Module
title: ollama
resource: src/llm/provider/ollama.rs#L1-L1804
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-providererror-result
  - external/super-r-trait-provider-providerstream
  - external/super-types
  - external/async-trait-async-trait
  - external/ollama-rs-error-ollamaerror-generation-chat-request-chatmessagerequest-chatmessage-chatmessagefinalresponsedata-chatmessageresponse-messagerole-generation-images-image-generation-parameters-formattype-jsonstructure-keepalive-thinktype-timeunit-generation-tools-toolcall-toolcallfunction-toolfunctioninfo-toolinfo-tooltype-models-modeloptions-ollama
  - external/futures-streamext-as
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [ModelOverrides](../../../../classes/src/llm/provider/ollama/ModelOverrides.md)
- [from_config](../../../../functions/src/llm/provider/ollama/ModelOverrides/from_config.md)
- [parse_think](../../../../functions/src/llm/provider/ollama/parse_think.md)
- [OllamaProvider](../../../../classes/src/llm/provider/ollama/OllamaProvider.md)
- [default_local](../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [new](../../../../functions/src/llm/provider/ollama/OllamaProvider/new.md)
- [with_per_model](../../../../functions/src/llm/provider/ollama/OllamaProvider/with_per_model.md)
- [with_think](../../../../functions/src/llm/provider/ollama/OllamaProvider/with_think.md)
- [overrides_for](../../../../functions/src/llm/provider/ollama/OllamaProvider/overrides_for.md)
- [with_default_model](../../../../functions/src/llm/provider/ollama/OllamaProvider/with_default_model.md)
- [with_keep_alive](../../../../functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive.md)
- [with_num_ctx](../../../../functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx.md)
- [with_sampling](../../../../functions/src/llm/provider/ollama/OllamaProvider/with_sampling.md)
- [to_ollama_request](../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [from_ollama_response](../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)
- [complete](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [supports_streaming](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/supports_streaming.md)
- [supports_tools](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/supports_tools.md)
- [supports_vision](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/supports_vision.md)
- [name](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/supported_models.md)
- [validate_model](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/validate_model.md)
- [context_window](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/calculate_cost.md)
- [collect_tool_calls](../../../../functions/src/llm/provider/ollama/collect_tool_calls.md)
- [stop_reason_for](../../../../functions/src/llm/provider/ollama/stop_reason_for.md)
- [maybe_tool_call_json](../../../../functions/src/llm/provider/ollama/maybe_tool_call_json.md)
- [tool_call_from_content](../../../../functions/src/llm/provider/ollama/tool_call_from_content.md)
- [fenced_json_blocks](../../../../functions/src/llm/provider/ollama/fenced_json_blocks.md)
- [parse_tool_call_object](../../../../functions/src/llm/provider/ollama/parse_tool_call_object.md)
- [to_ollama_tool](../../../../functions/src/llm/provider/ollama/to_ollama_tool.md)
- [to_ollama_format](../../../../functions/src/llm/provider/ollama/to_ollama_format.md)
- [parse_keep_alive](../../../../functions/src/llm/provider/ollama/parse_keep_alive.md)
- [perf_metrics_from_final_data](../../../../functions/src/llm/provider/ollama/perf_metrics_from_final_data.md)
- [map_ollama_error](../../../../functions/src/llm/provider/ollama/map_ollama_error.md)
- [test_ollama_provider_creation](../../../../functions/src/llm/provider/ollama/test_ollama_provider_creation.md)
- [test_with_default_model](../../../../functions/src/llm/provider/ollama/test_with_default_model.md)
- [per_model_override_wins_over_provider_default_for_that_model](../../../../functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model.md)
- [per_model_override_falls_back_field_by_field](../../../../functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field.md)
- [overrides_for_returns_provider_defaults_when_no_per_model_map](../../../../functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map.md)
- [context_window_reflects_the_per_model_num_ctx_that_is_actually_requested](../../../../functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested.md)
- [test_invalid_host_falls_back_to_default](../../../../functions/src/llm/provider/ollama/test_invalid_host_falls_back_to_default.md)
- [test_validate_model_always_true](../../../../functions/src/llm/provider/ollama/test_validate_model_always_true.md)
- [test_supported_models_includes_gemma4](../../../../functions/src/llm/provider/ollama/test_supported_models_includes_gemma4.md)
- [test_supported_models_includes_ornith](../../../../functions/src/llm/provider/ollama/test_supported_models_includes_ornith.md)
- [test_context_window_default_and_custom](../../../../functions/src/llm/provider/ollama/test_context_window_default_and_custom.md)
- [test_calculate_cost_is_always_zero](../../../../functions/src/llm/provider/ollama/test_calculate_cost_is_always_zero.md)
- [test_supports_vision_detection](../../../../functions/src/llm/provider/ollama/test_supports_vision_detection.md)
- [test_parse_keep_alive](../../../../functions/src/llm/provider/ollama/test_parse_keep_alive.md)
- [test_perf_metrics_from_final_data](../../../../functions/src/llm/provider/ollama/test_perf_metrics_from_final_data.md)
- [test_map_ollama_error_not_found](../../../../functions/src/llm/provider/ollama/test_map_ollama_error_not_found.md)
- [model_not_found_error_is_unwrapped_and_actionable](../../../../functions/src/llm/provider/ollama/model_not_found_error_is_unwrapped_and_actionable.md)
- [test_to_ollama_request_maps_common_fields](../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [mock_response](../../../../functions/src/llm/provider/ollama/mock_response.md)
- [bash_tool](../../../../functions/src/llm/provider/ollama/bash_tool.md)
- [tool_call_printed_as_content_is_recovered](../../../../functions/src/llm/provider/ollama/tool_call_printed_as_content_is_recovered.md)
- [tool_call_in_a_json_fence_is_recovered](../../../../functions/src/llm/provider/ollama/tool_call_in_a_json_fence_is_recovered.md)
- [tool_call_in_a_fence_embedded_in_prose_is_recovered](../../../../functions/src/llm/provider/ollama/tool_call_in_a_fence_embedded_in_prose_is_recovered.md)
- [first_of_several_fenced_calls_is_recovered](../../../../functions/src/llm/provider/ollama/first_of_several_fenced_calls_is_recovered.md)
- [fenced_non_tool_json_is_not_recovered](../../../../functions/src/llm/provider/ollama/fenced_non_tool_json_is_not_recovered.md)
- [prose_is_never_mistaken_for_a_tool_call](../../../../functions/src/llm/provider/ollama/prose_is_never_mistaken_for_a_tool_call.md)
- [only_json_like_content_is_withheld_from_streaming](../../../../functions/src/llm/provider/ollama/only_json_like_content_is_withheld_from_streaming.md)
- [recovered_tool_call_becomes_a_tool_use_block](../../../../functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block.md)
- [fenced_call_in_prose_becomes_a_tool_use_block](../../../../functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block.md)
- [from_ollama_response_plain_text_with_final_data](../../../../functions/src/llm/provider/ollama/from_ollama_response_plain_text_with_final_data.md)
- [from_ollama_response_without_final_data_has_zero_usage_and_no_perf](../../../../functions/src/llm/provider/ollama/from_ollama_response_without_final_data_has_zero_usage_and_no_perf.md)
- [from_ollama_response_extracts_tool_calls](../../../../functions/src/llm/provider/ollama/from_ollama_response_extracts_tool_calls.md)
- [streamed_tool_calls_arrive_before_the_done_chunk](../../../../functions/src/llm/provider/ollama/streamed_tool_calls_arrive_before_the_done_chunk.md)
- [stream_without_tool_calls_ends_the_turn](../../../../functions/src/llm/provider/ollama/stream_without_tool_calls_ends_the_turn.md)
- [streamed_tool_call_reaches_caller](../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)
- [from_ollama_response_uses_explicit_thinking_field](../../../../functions/src/llm/provider/ollama/from_ollama_response_uses_explicit_thinking_field.md)
- [from_ollama_response_falls_back_to_think_tags](../../../../functions/src/llm/provider/ollama/from_ollama_response_falls_back_to_think_tags.md)
- [to_ollama_tool_converts_valid_schema](../../../../functions/src/llm/provider/ollama/to_ollama_tool_converts_valid_schema.md)
- [to_ollama_tool_falls_back_on_invalid_schema](../../../../functions/src/llm/provider/ollama/to_ollama_tool_falls_back_on_invalid_schema.md)
- [to_ollama_format_json_object_marker](../../../../functions/src/llm/provider/ollama/to_ollama_format_json_object_marker.md)
- [to_ollama_format_structured_schema](../../../../functions/src/llm/provider/ollama/to_ollama_format_structured_schema.md)
- [to_ollama_request_maps_tool_messages](../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages.md)
- [to_ollama_request_maps_thinking_and_response_format](../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format.md)
- [per_model_think_false_is_sent_when_request_has_no_thinking](../../../../functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking.md)
- [request_thinking_wins_over_configured_think](../../../../functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think.md)
- [invalid_think_value_is_ignored](../../../../functions/src/llm/provider/ollama/invalid_think_value_is_ignored.md)
- [to_ollama_request_embeds_base64_image](../../../../functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image.md)

# Imports

- `super::error::{ProviderError, Result}`
- `super::r#trait::{Provider, ProviderStream}`
- `super::types::*`
- `async_trait::async_trait`
- `ollama_rs::{
    error::OllamaError,
    generation::chat::{
        request::ChatMessageRequest, ChatMessage, ChatMessageFinalResponseData,
        ChatMessageResponse, MessageRole,
    },
    generation::images::Image,
    generation::parameters::{FormatType, JsonStructure, KeepAlive, ThinkType, TimeUnit},
    generation::tools::{ToolCall, ToolCallFunction, ToolFunctionInfo, ToolInfo, ToolType},
    models::ModelOptions,
    Ollama,
}`
- `futures::StreamExt as _`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)