---
type: Rust Method
title: local
resource: src/llm/provider/qwen.rs#L133-L145
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/build_client
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/factory/try_create_qwen
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/configure_qwen_auto_selects_openai_parser_for_coder_next
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/configure_qwen_keeps_hermes_default_for_other_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/streaming_assembles_hermes_tool_call_from_buffered_text
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/streaming_plain_text_roundtrips_without_tool_calls
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_local_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_tool_parser_configuration
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_thinking_mode_configuration
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_hermes_tool_call_parsing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_multiple_hermes_tool_calls
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_hermes_malformed_json_is_skipped_without_panicking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_hermes_json_missing_required_fields_is_skipped
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_thinking_extraction
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_calculate_cost_local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_custom_default_model
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
  - target: functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_hermes_tools_format
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_native_qwen_parser_configuration
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_native_qwen_tools_format
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_native_qwen_result_format
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_clean_incomplete_markers
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_fallback_parses_bare_json_tool_call
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_fallback_rejects_unregistered_tool_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_fallback_parses_fenced_json_tool_call
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_fallback_does_not_corrupt_unrelated_fenced_code_block
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_fallback_ignores_unrelated_json
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_uses_fallback_when_no_hermes_tags
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_openai_parser_still_detects_fallback_json
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn local(base_url: String) -> Self`

# Calls

- [build_client](../../../../../../functions/src/llm/provider/qwen/QwenProvider/build_client.md)

# Called by

- [try_create_qwen](../../../../../../functions/src/llm/provider/factory/try_create_qwen.md)
- [configure_qwen_auto_selects_openai_parser_for_coder_next](../../../../../../functions/src/llm/provider/factory/configure_qwen_auto_selects_openai_parser_for_coder_next.md)
- [configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection](../../../../../../functions/src/llm/provider/factory/configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection.md)
- [configure_qwen_keeps_hermes_default_for_other_models](../../../../../../functions/src/llm/provider/factory/configure_qwen_keeps_hermes_default_for_other_models.md)
- [streaming_assembles_hermes_tool_call_from_buffered_text](../../../../../../functions/src/llm/provider/qwen/streaming_assembles_hermes_tool_call_from_buffered_text.md)
- [streaming_plain_text_roundtrips_without_tool_calls](../../../../../../functions/src/llm/provider/qwen/streaming_plain_text_roundtrips_without_tool_calls.md)
- [stream_assembles_openai_style_tool_call_across_sse_chunks](../../../../../../functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks.md)
- [stream_skips_malformed_sse_chunk_and_continues](../../../../../../functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues.md)
- [test_local_provider_creation](../../../../../../functions/src/llm/provider/qwen/test_local_provider_creation.md)
- [test_tool_parser_configuration](../../../../../../functions/src/llm/provider/qwen/test_tool_parser_configuration.md)
- [test_thinking_mode_configuration](../../../../../../functions/src/llm/provider/qwen/test_thinking_mode_configuration.md)
- [test_hermes_tool_call_parsing](../../../../../../functions/src/llm/provider/qwen/test_hermes_tool_call_parsing.md)
- [test_multiple_hermes_tool_calls](../../../../../../functions/src/llm/provider/qwen/test_multiple_hermes_tool_calls.md)
- [test_hermes_malformed_json_is_skipped_without_panicking](../../../../../../functions/src/llm/provider/qwen/test_hermes_malformed_json_is_skipped_without_panicking.md)
- [test_hermes_json_missing_required_fields_is_skipped](../../../../../../functions/src/llm/provider/qwen/test_hermes_json_missing_required_fields_is_skipped.md)
- [test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display.md)
- [test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever.md)
- [test_thinking_extraction](../../../../../../functions/src/llm/provider/qwen/test_thinking_extraction.md)
- [test_thinking_extraction_out_of_order_tags_does_not_panic](../../../../../../functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic.md)
- [test_calculate_cost_local](../../../../../../functions/src/llm/provider/qwen/test_calculate_cost_local.md)
- [test_custom_default_model](../../../../../../functions/src/llm/provider/qwen/test_custom_default_model.md)
- [test_sampling_defaults_qwen25_coder_local](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen25_coder_local.md)
- [test_text_sent_with_tool_results_follows_them](../../../../../../functions/src/llm/provider/qwen/test_text_sent_with_tool_results_follows_them.md)
- [test_sampling_defaults_qwen3_non_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_non_thinking.md)
- [test_sampling_defaults_qwen3_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking.md)
- [test_sampling_explicit_request_top_p_wins](../../../../../../functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins.md)
- [test_hermes_tools_format](../../../../../../functions/src/llm/provider/qwen/test_hermes_tools_format.md)
- [test_native_qwen_parser_configuration](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_parser_configuration.md)
- [test_native_qwen_tool_call_parsing](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing.md)
- [test_multiple_native_qwen_tool_calls](../../../../../../functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls.md)
- [test_native_qwen_tools_format](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_tools_format.md)
- [test_native_qwen_result_format](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_result_format.md)
- [test_clean_incomplete_markers](../../../../../../functions/src/llm/provider/qwen/test_clean_incomplete_markers.md)
- [test_fallback_parses_bare_json_tool_call](../../../../../../functions/src/llm/provider/qwen/test_fallback_parses_bare_json_tool_call.md)
- [test_fallback_rejects_unregistered_tool_name](../../../../../../functions/src/llm/provider/qwen/test_fallback_rejects_unregistered_tool_name.md)
- [test_fallback_parses_fenced_json_tool_call](../../../../../../functions/src/llm/provider/qwen/test_fallback_parses_fenced_json_tool_call.md)
- [test_fallback_does_not_corrupt_unrelated_fenced_code_block](../../../../../../functions/src/llm/provider/qwen/test_fallback_does_not_corrupt_unrelated_fenced_code_block.md)
- [test_fallback_ignores_unrelated_json](../../../../../../functions/src/llm/provider/qwen/test_fallback_ignores_unrelated_json.md)
- [test_from_qwen_response_uses_fallback_when_no_hermes_tags](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_uses_fallback_when_no_hermes_tags.md)
- [test_from_qwen_response_openai_parser_still_detects_fallback_json](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_openai_parser_still_detects_fallback_json.md)
- [test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call.md)
- [test_sampling_defaults_unrecognized_model_name_is_conservative](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative.md)