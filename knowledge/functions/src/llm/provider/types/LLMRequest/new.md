---
type: Rust Method
title: new
resource: src/llm/provider/types.rs#L157-L175
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/build_request
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/test_failover_on_rate_limit_tries_next_provider
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/test_failover_sends_the_secondary_its_own_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/test_failover_all_fail_returns_last_error
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
  - target: functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_without_the_feature_is_always_none
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/to_ollama_request_keeps_text_sent_with_tool_results
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/invalid_think_value_is_ignored
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_llm_request_new_fields
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen25_coder_local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_text_sent_with_tool_results_follows_them
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_non_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/test_llm_request_builder
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/with_thinking_sets_temperature_and_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/with_thinking_sets_temperature
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/with_thinking_zero_budget_is_noop
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/consolidate/MemoryStore/lesson_from_episodes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/thinking_config_forces_temperature_one
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/zero_budget_does_not_enable_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_basic
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_single_chunk
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_multiple_chunks
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_token_counting
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_stop_reason
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_error_handling
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_empty_response
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_content_accumulation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/streaming_test/test_streaming_request_builder
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(model: impl Into<String>, messages: Vec<Message>) -> Self`

# Called by

- [build_request](../../../../../../functions/src/llm/agent/service/AgentService/build_request.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [test_failover_on_rate_limit_tries_next_provider](../../../../../../functions/src/llm/provider/factory/test_failover_on_rate_limit_tries_next_provider.md)
- [test_failover_sends_the_secondary_its_own_model](../../../../../../functions/src/llm/provider/factory/test_failover_sends_the_secondary_its_own_model.md)
- [test_failover_all_fail_returns_last_error](../../../../../../functions/src/llm/provider/factory/test_failover_all_fail_returns_last_error.md)
- [test_to_gemini_request_maps_system_and_tools](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools.md)
- [test_to_gemini_request_tool_result_uses_function_name](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_tool_result_uses_function_name.md)
- [test_to_gemini_request_keeps_text_sent_with_tool_results](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_keeps_text_sent_with_tool_results.md)
- [test_thinking_config_forwarded](../../../../../../functions/src/llm/provider/gemini/test_thinking_config_forwarded.md)
- [test_json_mode_sets_response_mime_type](../../../../../../functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type.md)
- [test_full_json_schema_sets_response_schema](../../../../../../functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema.md)
- [test_inline_image_becomes_inline_data_part](../../../../../../functions/src/llm/provider/gemini/test_inline_image_becomes_inline_data_part.md)
- [test_image_url_source_is_skipped_without_panicking](../../../../../../functions/src/llm/provider/gemini/test_image_url_source_is_skipped_without_panicking.md)
- [build_sampler_seed_offset_changes_the_resolved_seed](../../../../../../functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed.md)
- [try_build_constrained_sampler_succeeds_for_a_valid_tool_schema](../../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema.md)
- [try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay](../../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay.md)
- [try_build_constrained_sampler_without_the_feature_is_always_none](../../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_without_the_feature_is_always_none.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [streamed_tool_call_reaches_caller](../../../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)
- [to_ollama_request_maps_tool_messages](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages.md)
- [to_ollama_request_keeps_text_sent_with_tool_results](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_keeps_text_sent_with_tool_results.md)
- [to_ollama_request_maps_thinking_and_response_format](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format.md)
- [per_model_think_false_is_sent_when_request_has_no_thinking](../../../../../../functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking.md)
- [request_thinking_wins_over_configured_think](../../../../../../functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think.md)
- [invalid_think_value_is_ignored](../../../../../../functions/src/llm/provider/ollama/invalid_think_value_is_ignored.md)
- [to_ollama_request_embeds_base64_image](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image.md)
- [text_arrives_before_the_response_is_complete](../../../../../../functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete.md)
- [a_reply_cut_off_before_the_done_chunk_is_an_error](../../../../../../functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error.md)
- [test_llm_request_new_fields](../../../../../../functions/src/llm/provider/openai/test_llm_request_new_fields.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)
- [test_text_sent_with_tool_results_follows_them](../../../../../../functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them.md)
- [text_arrives_before_the_response_is_complete](../../../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)
- [stream_assembles_openai_style_tool_call_across_sse_chunks](../../../../../../functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks.md)
- [stream_skips_malformed_sse_chunk_and_continues](../../../../../../functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues.md)
- [test_sampling_defaults_qwen25_coder_local](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen25_coder_local.md)
- [test_text_sent_with_tool_results_follows_them](../../../../../../functions/src/llm/provider/qwen/test_text_sent_with_tool_results_follows_them.md)
- [test_sampling_defaults_qwen3_non_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_non_thinking.md)
- [test_sampling_defaults_qwen3_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking.md)
- [test_sampling_defaults_dashscope_omits_vendor_extensions](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions.md)
- [test_sampling_explicit_request_top_p_wins](../../../../../../functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins.md)
- [test_sampling_config_override_wins_over_defaults](../../../../../../functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults.md)
- [test_sampling_defaults_unrecognized_model_name_is_conservative](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)
- [with_thinking_sets_temperature_and_config](../../../../../../functions/src/llm/provider/types/with_thinking_sets_temperature_and_config.md)
- [with_thinking_sets_temperature](../../../../../../functions/src/llm/provider/types/with_thinking_sets_temperature.md)
- [with_thinking_zero_budget_is_noop](../../../../../../functions/src/llm/provider/types/with_thinking_zero_budget_is_noop.md)
- [lesson_from_episodes](../../../../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)
- [extract](../../../../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [thinking_config_forces_temperature_one](../../../../../../functions/tests/model_routing_test/thinking_config_forces_temperature_one.md)
- [zero_budget_does_not_enable_thinking](../../../../../../functions/tests/model_routing_test/zero_budget_does_not_enable_thinking.md)
- [test_streaming_basic](../../../../../../functions/tests/streaming_test/test_streaming_basic.md)
- [test_streaming_single_chunk](../../../../../../functions/tests/streaming_test/test_streaming_single_chunk.md)
- [test_streaming_multiple_chunks](../../../../../../functions/tests/streaming_test/test_streaming_multiple_chunks.md)
- [test_streaming_token_counting](../../../../../../functions/tests/streaming_test/test_streaming_token_counting.md)
- [test_streaming_stop_reason](../../../../../../functions/tests/streaming_test/test_streaming_stop_reason.md)
- [test_streaming_error_handling](../../../../../../functions/tests/streaming_test/test_streaming_error_handling.md)
- [test_streaming_empty_response](../../../../../../functions/tests/streaming_test/test_streaming_empty_response.md)
- [test_streaming_content_accumulation](../../../../../../functions/tests/streaming_test/test_streaming_content_accumulation.md)
- [test_streaming_request_builder](../../../../../../functions/tests/streaming_test/test_streaming_request_builder.md)