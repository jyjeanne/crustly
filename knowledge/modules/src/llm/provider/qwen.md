---
type: Rust Module
title: qwen
resource: src/llm/provider/qwen.rs#L1-L2831
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-providererror-result
  - external/super-r-trait-provider-providerstream
  - external/super-types
  - external/async-trait-async-trait
  - external/futures-stream-streamext
  - external/reqwest-client
  - external/serde-deserialize-serialize
  - external/std-time-duration
  - external/super-retry-retry-with-backoff-retryconfig
  - external/super
  - external/tokio-io-asyncreadext-asyncwriteext
  member_of:
  - packages/crustly
---

# Contains

- [tool_call_index_in_bounds](../../../../functions/src/llm/provider/qwen/tool_call_index_in_bounds.md)
- [ToolCallParser](../../../../classes/src/llm/provider/qwen/ToolCallParser.md)
- [ThinkingConfig](../../../../classes/src/llm/provider/qwen/ThinkingConfig.md)
- [SamplingOverrides](../../../../classes/src/llm/provider/qwen/SamplingOverrides.md)
- [find_after](../../../../functions/src/llm/provider/qwen/find_after.md)
- [QwenProvider](../../../../classes/src/llm/provider/qwen/QwenProvider.md)
- [dashscope_intl](../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_intl.md)
- [dashscope_cn](../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_cn.md)
- [local](../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_base_url](../../../../functions/src/llm/provider/qwen/QwenProvider/with_base_url.md)
- [with_default_model](../../../../functions/src/llm/provider/qwen/QwenProvider/with_default_model.md)
- [with_tool_parser](../../../../functions/src/llm/provider/qwen/QwenProvider/with_tool_parser.md)
- [tool_parser](../../../../functions/src/llm/provider/qwen/QwenProvider/tool_parser.md)
- [with_sampling](../../../../functions/src/llm/provider/qwen/QwenProvider/with_sampling.md)
- [with_thinking](../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking.md)
- [with_thinking_budget](../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking_budget.md)
- [build_client](../../../../functions/src/llm/provider/qwen/QwenProvider/build_client.md)
- [is_local](../../../../functions/src/llm/provider/qwen/QwenProvider/is_local.md)
- [generate_call_id](../../../../functions/src/llm/provider/qwen/QwenProvider/generate_call_id.md)
- [headers](../../../../functions/src/llm/provider/qwen/QwenProvider/headers.md)
- [format_hermes_tools](../../../../functions/src/llm/provider/qwen/QwenProvider/format_hermes_tools.md)
- [parse_hermes_tool_calls](../../../../functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls.md)
- [find_json_objects](../../../../functions/src/llm/provider/qwen/QwenProvider/find_json_objects.md)
- [expand_span_over_adjacent_fences](../../../../functions/src/llm/provider/qwen/QwenProvider/expand_span_over_adjacent_fences.md)
- [parse_fallback_tool_calls](../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)
- [extract_thinking](../../../../functions/src/llm/provider/qwen/QwenProvider/extract_thinking.md)
- [format_native_qwen_tools](../../../../functions/src/llm/provider/qwen/QwenProvider/format_native_qwen_tools.md)
- [parse_native_qwen_tool_calls](../../../../functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls.md)
- [format_native_qwen_result](../../../../functions/src/llm/provider/qwen/QwenProvider/format_native_qwen_result.md)
- [clean_incomplete_markers](../../../../functions/src/llm/provider/qwen/QwenProvider/clean_incomplete_markers.md)
- [to_qwen_request](../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [local_only](../../../../functions/src/llm/provider/qwen/QwenProvider/local_only.md)
- [default_sampling](../../../../functions/src/llm/provider/qwen/QwenProvider/default_sampling.md)
- [push_fallback_or_text](../../../../functions/src/llm/provider/qwen/QwenProvider/push_fallback_or_text.md)
- [from_qwen_response](../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [handle_error](../../../../functions/src/llm/provider/qwen/QwenProvider/handle_error.md)
- [complete](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [supports_streaming](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/supports_streaming.md)
- [supports_tools](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/supports_tools.md)
- [supports_vision](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/supports_vision.md)
- [name](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/supported_models.md)
- [validate_model](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/validate_model.md)
- [context_window](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/qwen/QwenProvider/provider/calculate_cost.md)
- [llm_response_to_stream_events](../../../../functions/src/llm/provider/qwen/llm_response_to_stream_events.md)
- [QwenRequest](../../../../classes/src/llm/provider/qwen/QwenRequest.md)
- [QwenMessage](../../../../classes/src/llm/provider/qwen/QwenMessage.md)
- [QwenToolCall](../../../../classes/src/llm/provider/qwen/QwenToolCall.md)
- [QwenFunctionCall](../../../../classes/src/llm/provider/qwen/QwenFunctionCall.md)
- [QwenTool](../../../../classes/src/llm/provider/qwen/QwenTool.md)
- [QwenFunction](../../../../classes/src/llm/provider/qwen/QwenFunction.md)
- [QwenResponse](../../../../classes/src/llm/provider/qwen/QwenResponse.md)
- [QwenChoice](../../../../classes/src/llm/provider/qwen/QwenChoice.md)
- [QwenUsage](../../../../classes/src/llm/provider/qwen/QwenUsage.md)
- [QwenStreamChunk](../../../../classes/src/llm/provider/qwen/QwenStreamChunk.md)
- [QwenStreamChoice](../../../../classes/src/llm/provider/qwen/QwenStreamChoice.md)
- [QwenMessageDelta](../../../../classes/src/llm/provider/qwen/QwenMessageDelta.md)
- [QwenToolCallDelta](../../../../classes/src/llm/provider/qwen/QwenToolCallDelta.md)
- [QwenFunctionCallDelta](../../../../classes/src/llm/provider/qwen/QwenFunctionCallDelta.md)
- [QwenErrorResponse](../../../../classes/src/llm/provider/qwen/QwenErrorResponse.md)
- [QwenError](../../../../classes/src/llm/provider/qwen/QwenError.md)
- [find_after_ignores_a_match_before_start](../../../../functions/src/llm/provider/qwen/find_after_ignores_a_match_before_start.md)
- [find_after_returns_none_when_nothing_matches_after_start](../../../../functions/src/llm/provider/qwen/find_after_returns_none_when_nothing_matches_after_start.md)
- [find_after_returns_an_absolute_offset_not_a_relative_one](../../../../functions/src/llm/provider/qwen/find_after_returns_an_absolute_offset_not_a_relative_one.md)
- [stream_events_from_buffered_content](../../../../functions/src/llm/provider/qwen/stream_events_from_buffered_content.md)
- [streaming_assembles_hermes_tool_call_from_buffered_text](../../../../functions/src/llm/provider/qwen/streaming_assembles_hermes_tool_call_from_buffered_text.md)
- [streaming_plain_text_roundtrips_without_tool_calls](../../../../functions/src/llm/provider/qwen/streaming_plain_text_roundtrips_without_tool_calls.md)
- [mock_sse_server](../../../../functions/src/llm/provider/qwen/mock_sse_server.md)
- [stream_assembles_openai_style_tool_call_across_sse_chunks](../../../../functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks.md)
- [stream_skips_malformed_sse_chunk_and_continues](../../../../functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues.md)
- [test_qwen_provider_creation](../../../../functions/src/llm/provider/qwen/test_qwen_provider_creation.md)
- [test_local_provider_creation](../../../../functions/src/llm/provider/qwen/test_local_provider_creation.md)
- [test_tool_parser_configuration](../../../../functions/src/llm/provider/qwen/test_tool_parser_configuration.md)
- [test_thinking_mode_configuration](../../../../functions/src/llm/provider/qwen/test_thinking_mode_configuration.md)
- [test_hermes_tool_call_parsing](../../../../functions/src/llm/provider/qwen/test_hermes_tool_call_parsing.md)
- [test_multiple_hermes_tool_calls](../../../../functions/src/llm/provider/qwen/test_multiple_hermes_tool_calls.md)
- [test_hermes_malformed_json_is_skipped_without_panicking](../../../../functions/src/llm/provider/qwen/test_hermes_malformed_json_is_skipped_without_panicking.md)
- [test_hermes_json_missing_required_fields_is_skipped](../../../../functions/src/llm/provider/qwen/test_hermes_json_missing_required_fields_is_skipped.md)
- [test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display](../../../../functions/src/llm/provider/qwen/test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display.md)
- [test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever](../../../../functions/src/llm/provider/qwen/test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever.md)
- [test_thinking_extraction](../../../../functions/src/llm/provider/qwen/test_thinking_extraction.md)
- [test_thinking_extraction_out_of_order_tags_does_not_panic](../../../../functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic.md)
- [test_supported_models](../../../../functions/src/llm/provider/qwen/test_supported_models.md)
- [test_context_window](../../../../functions/src/llm/provider/qwen/test_context_window.md)
- [test_calculate_cost_local](../../../../functions/src/llm/provider/qwen/test_calculate_cost_local.md)
- [test_calculate_cost_cloud](../../../../functions/src/llm/provider/qwen/test_calculate_cost_cloud.md)
- [test_calculate_cost_unknown_cloud_model_returns_zero](../../../../functions/src/llm/provider/qwen/test_calculate_cost_unknown_cloud_model_returns_zero.md)
- [test_tool_call_index_in_bounds](../../../../functions/src/llm/provider/qwen/test_tool_call_index_in_bounds.md)
- [test_custom_default_model](../../../../functions/src/llm/provider/qwen/test_custom_default_model.md)
- [test_sampling_defaults_qwen25_coder_local](../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen25_coder_local.md)
- [test_sampling_defaults_qwen3_non_thinking](../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_non_thinking.md)
- [test_sampling_defaults_qwen3_thinking](../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking.md)
- [test_sampling_defaults_dashscope_omits_vendor_extensions](../../../../functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions.md)
- [test_sampling_explicit_request_top_p_wins](../../../../functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins.md)
- [test_sampling_config_override_wins_over_defaults](../../../../functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults.md)
- [test_hermes_tools_format](../../../../functions/src/llm/provider/qwen/test_hermes_tools_format.md)
- [test_native_qwen_parser_configuration](../../../../functions/src/llm/provider/qwen/test_native_qwen_parser_configuration.md)
- [test_native_qwen_tool_call_parsing](../../../../functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing.md)
- [test_multiple_native_qwen_tool_calls](../../../../functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls.md)
- [test_native_qwen_tools_format](../../../../functions/src/llm/provider/qwen/test_native_qwen_tools_format.md)
- [test_native_qwen_result_format](../../../../functions/src/llm/provider/qwen/test_native_qwen_result_format.md)
- [test_clean_incomplete_markers](../../../../functions/src/llm/provider/qwen/test_clean_incomplete_markers.md)
- [test_fallback_parses_bare_json_tool_call](../../../../functions/src/llm/provider/qwen/test_fallback_parses_bare_json_tool_call.md)
- [test_fallback_rejects_unregistered_tool_name](../../../../functions/src/llm/provider/qwen/test_fallback_rejects_unregistered_tool_name.md)
- [test_fallback_parses_fenced_json_tool_call](../../../../functions/src/llm/provider/qwen/test_fallback_parses_fenced_json_tool_call.md)
- [test_fallback_does_not_corrupt_unrelated_fenced_code_block](../../../../functions/src/llm/provider/qwen/test_fallback_does_not_corrupt_unrelated_fenced_code_block.md)
- [test_fallback_ignores_unrelated_json](../../../../functions/src/llm/provider/qwen/test_fallback_ignores_unrelated_json.md)
- [test_find_json_objects_recovers_nested_object_after_failed_outer_parse](../../../../functions/src/llm/provider/qwen/test_find_json_objects_recovers_nested_object_after_failed_outer_parse.md)
- [test_from_qwen_response_uses_fallback_when_no_hermes_tags](../../../../functions/src/llm/provider/qwen/test_from_qwen_response_uses_fallback_when_no_hermes_tags.md)
- [test_from_qwen_response_openai_parser_still_detects_fallback_json](../../../../functions/src/llm/provider/qwen/test_from_qwen_response_openai_parser_still_detects_fallback_json.md)
- [test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call](../../../../functions/src/llm/provider/qwen/test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call.md)
- [test_sampling_defaults_unrecognized_model_name_is_conservative](../../../../functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative.md)
- [test_stop_words_defined](../../../../functions/src/llm/provider/qwen/test_stop_words_defined.md)

# Imports

- `super::error::{ProviderError, Result}`
- `super::r#trait::{Provider, ProviderStream}`
- `super::types::*`
- `async_trait::async_trait`
- `futures::stream::StreamExt`
- `reqwest::Client`
- `serde::{Deserialize, Serialize}`
- `std::time::Duration`
- `super::retry::{retry_with_backoff, RetryConfig}`
- `super::*`
- `tokio::io::{AsyncReadExt, AsyncWriteExt}`

# Member of

- [crustly](../../../../packages/crustly.md)