---
type: Rust Method
title: user
resource: src/llm/provider/types.rs#L31-L36
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/test_add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/test_would_exceed_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/test_usage_percentage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/test_trim_to_fit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/user_turn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/finish
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools
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
  - target: functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/invalid_think_value_is_ignored
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/test_llm_request_new_fields
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen25_coder_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_non_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/test_message_creation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/test_llm_request_builder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/lesson_from_episodes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn user(text: impl Into<String>) -> Self`

# Called by

- [elide_tool_results_keeps_recent_output_and_frees_tokens](../../../../../../functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens.md)
- [elide_tool_results_leaves_short_output_alone](../../../../../../functions/src/llm/agent/compaction/elide_tool_results_leaves_short_output_alone.md)
- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)
- [compaction_refuses_when_nothing_would_be_dropped](../../../../../../functions/src/llm/agent/compaction/compaction_refuses_when_nothing_would_be_dropped.md)
- [test_add_message](../../../../../../functions/src/llm/agent/context/test_add_message.md)
- [test_would_exceed_limit](../../../../../../functions/src/llm/agent/context/test_would_exceed_limit.md)
- [test_usage_percentage](../../../../../../functions/src/llm/agent/context/test_usage_percentage.md)
- [test_trim_to_fit](../../../../../../functions/src/llm/agent/context/test_trim_to_fit.md)
- [user_turn](../../../../../../functions/src/llm/agent/service/user_turn.md)
- [make_room_elides_old_tool_output_when_the_budget_is_tight](../../../../../../functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight.md)
- [streamed_ollama_tool_call_survives_drain](../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)
- [test_to_gemini_request_maps_system_and_tools](../../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_maps_system_and_tools.md)
- [test_thinking_config_forwarded](../../../../../../functions/src/llm/provider/gemini/test_thinking_config_forwarded.md)
- [test_json_mode_sets_response_mime_type](../../../../../../functions/src/llm/provider/gemini/test_json_mode_sets_response_mime_type.md)
- [test_full_json_schema_sets_response_schema](../../../../../../functions/src/llm/provider/gemini/test_full_json_schema_sets_response_schema.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [streamed_tool_call_reaches_caller](../../../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)
- [to_ollama_request_maps_thinking_and_response_format](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format.md)
- [per_model_think_false_is_sent_when_request_has_no_thinking](../../../../../../functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking.md)
- [request_thinking_wins_over_configured_think](../../../../../../functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think.md)
- [invalid_think_value_is_ignored](../../../../../../functions/src/llm/provider/ollama/invalid_think_value_is_ignored.md)
- [text_arrives_before_the_response_is_complete](../../../../../../functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete.md)
- [a_reply_cut_off_before_the_done_chunk_is_an_error](../../../../../../functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error.md)
- [test_llm_request_new_fields](../../../../../../functions/src/llm/provider/openai/test_llm_request_new_fields.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)
- [text_arrives_before_the_response_is_complete](../../../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)
- [stream_assembles_openai_style_tool_call_across_sse_chunks](../../../../../../functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks.md)
- [stream_skips_malformed_sse_chunk_and_continues](../../../../../../functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues.md)
- [test_sampling_defaults_qwen25_coder_local](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen25_coder_local.md)
- [test_sampling_defaults_qwen3_non_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_non_thinking.md)
- [test_sampling_defaults_qwen3_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking.md)
- [test_sampling_defaults_dashscope_omits_vendor_extensions](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions.md)
- [test_sampling_explicit_request_top_p_wins](../../../../../../functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins.md)
- [test_sampling_config_override_wins_over_defaults](../../../../../../functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults.md)
- [test_sampling_defaults_unrecognized_model_name_is_conservative](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative.md)
- [test_message_creation](../../../../../../functions/src/llm/provider/types/test_message_creation.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)
- [lesson_from_episodes](../../../../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)
- [extract](../../../../../../functions/src/memory/extraction/MemoryStore/extract.md)