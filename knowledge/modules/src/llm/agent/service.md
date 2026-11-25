---
type: Rust Module
title: service
resource: src/llm/agent/service.rs#L1-L4301
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-budget-calibration-from-tool-schema-tokens-contextbudget-contextusage-default-context-window
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-compaction
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-context-token-count-agentcontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-error-agenterror-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-middleware-tool-call-signature
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-middleware-has-mutating-capability-limits-loopguard-pipeline-toolcallrecord-verification
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-tool-profile-toolselection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-toolprofile
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-router-modelrouter
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-contentblock-contentdelta-llmrequest-llmresponse-message-perfmetrics-provider-providerstream-role-stopreason-streamevent-tokenusage-tool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-cache-toolresultcache-toolttlconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-filereadcache-toolexecutioncontext-toolregistry
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-memorystore-recallrequest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-services-messageservice-servicecontext-sessionservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/futures-streamext-as
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-future-future
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-pin-pin
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-mpsc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-util-sync-cancellationtoken
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-providererror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-llmrequest-llmresponse-tokenusage
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-toolcapability
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-ollamaprovider-provider-tool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-types-messagedelta
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-contentdelta
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-subagentlauncher-subagentstatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-subagentlauncher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memoryscope
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memorycategory-memorysource-newmemory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-forget-memory-forgetmemorytool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memorycategory-memoryscope-memorysource-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-save-memory-savememorytool
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [FinalText](../../../../classes/src/llm/agent/service/FinalText.md)
- [RequestTools](../../../../classes/src/llm/agent/service/RequestTools.md)
- [new](../../../../functions/src/llm/agent/service/RequestTools/new.md)
- [plan_completion_rejection](../../../../functions/src/llm/agent/service/plan_completion_rejection.md)
- [is_parallelizable](../../../../functions/src/llm/agent/service/is_parallelizable.md)
- [ToolApprovalInfo](../../../../classes/src/llm/agent/service/ToolApprovalInfo.md)
- [AgentService](../../../../classes/src/llm/agent/service/AgentService.md)
- [Fallback](../../../../classes/src/llm/agent/service/Fallback.md)
- [route_text_delta](../../../../functions/src/llm/agent/service/route_text_delta.md)
- [apply_streamed_tool_input](../../../../functions/src/llm/agent/service/apply_streamed_tool_input.md)
- [drain_stream_to_response](../../../../functions/src/llm/agent/service/drain_stream_to_response.md)
- [user_turn](../../../../functions/src/llm/agent/service/user_turn.md)
- [new](../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_tool_allowlist](../../../../functions/src/llm/agent/service/AgentService/with_tool_allowlist.md)
- [with_fallback](../../../../functions/src/llm/agent/service/AgentService/with_fallback.md)
- [with_subagents](../../../../functions/src/llm/agent/service/AgentService/with_subagents.md)
- [with_request_limits](../../../../functions/src/llm/agent/service/AgentService/with_request_limits.md)
- [with_verification](../../../../functions/src/llm/agent/service/AgentService/with_verification.md)
- [with_memory](../../../../functions/src/llm/agent/service/AgentService/with_memory.md)
- [memory_block](../../../../functions/src/llm/agent/service/AgentService/memory_block.md)
- [with_project_notes](../../../../functions/src/llm/agent/service/AgentService/with_project_notes.md)
- [with_progress_file](../../../../functions/src/llm/agent/service/AgentService/with_progress_file.md)
- [with_tool_profile](../../../../functions/src/llm/agent/service/AgentService/with_tool_profile.md)
- [with_core_system_prompt](../../../../functions/src/llm/agent/service/AgentService/with_core_system_prompt.md)
- [with_model_router](../../../../functions/src/llm/agent/service/AgentService/with_model_router.md)
- [with_pool](../../../../functions/src/llm/agent/service/AgentService/with_pool.md)
- [with_system_prompt](../../../../functions/src/llm/agent/service/AgentService/with_system_prompt.md)
- [with_max_tool_iterations](../../../../functions/src/llm/agent/service/AgentService/with_max_tool_iterations.md)
- [with_tool_registry](../../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_auto_approve_tools](../../../../functions/src/llm/agent/service/AgentService/with_auto_approve_tools.md)
- [with_approval_callback](../../../../functions/src/llm/agent/service/AgentService/with_approval_callback.md)
- [with_working_directory](../../../../functions/src/llm/agent/service/AgentService/with_working_directory.md)
- [system_prompt_with_env](../../../../functions/src/llm/agent/service/AgentService/system_prompt_with_env.md)
- [system_prompt_for](../../../../functions/src/llm/agent/service/AgentService/system_prompt_for.md)
- [with_env](../../../../functions/src/llm/agent/service/AgentService/with_env.md)
- [with_allow_sub_agents](../../../../functions/src/llm/agent/service/AgentService/with_allow_sub_agents.md)
- [set_provider](../../../../functions/src/llm/agent/service/AgentService/set_provider.md)
- [provider_name](../../../../functions/src/llm/agent/service/AgentService/provider_name.md)
- [provider_model](../../../../functions/src/llm/agent/service/AgentService/provider_model.md)
- [provider_context_window](../../../../functions/src/llm/agent/service/AgentService/provider_context_window.md)
- [tool_selection](../../../../functions/src/llm/agent/service/AgentService/tool_selection.md)
- [exposed_tool_definitions](../../../../functions/src/llm/agent/service/AgentService/exposed_tool_definitions.md)
- [default_context_window](../../../../functions/src/llm/agent/service/AgentService/default_context_window.md)
- [exposed_tool_names](../../../../functions/src/llm/agent/service/AgentService/exposed_tool_names.md)
- [context_usage_preview](../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [calibration](../../../../functions/src/llm/agent/service/AgentService/calibration.md)
- [record_calibration](../../../../functions/src/llm/agent/service/AgentService/record_calibration.md)
- [measure_budget](../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [make_room](../../../../functions/src/llm/agent/service/AgentService/make_room.md)
- [round_pipeline](../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)
- [log_progress](../../../../functions/src/llm/agent/service/AgentService/log_progress.md)
- [build_request](../../../../functions/src/llm/agent/service/AgentService/build_request.md)
- [send_message](../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_streaming](../../../../functions/src/llm/agent/service/AgentService/send_message_streaming.md)
- [send_message_with_tools](../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [send_message_with_tools_and_mode](../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode.md)
- [send_message_with_tools_cancellable](../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_cancellable.md)
- [child_template](../../../../functions/src/llm/agent/service/AgentService/child_template.md)
- [send_message_with_tools_and_mode_streaming](../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_and_mode_streaming.md)
- [send_message_with_tools_inner](../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [call_provider_streaming](../../../../functions/src/llm/agent/service/AgentService/call_provider_streaming.md)
- [prepare_message_context](../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [extract_text_from_response](../../../../functions/src/llm/agent/service/AgentService/extract_text_from_response.md)
- [final_text_and_thinking](../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)
- [extract_thinking_from_response](../../../../functions/src/llm/agent/service/AgentService/extract_thinking_from_response.md)
- [AgentResponse](../../../../classes/src/llm/agent/service/AgentResponse.md)
- [AgentStreamResponse](../../../../classes/src/llm/agent/service/AgentStreamResponse.md)
- [MockProvider](../../../../classes/src/llm/agent/service/MockProvider.md)
- [complete](../../../../functions/src/llm/agent/service/MockProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/agent/service/MockProvider/provider/stream.md)
- [name](../../../../functions/src/llm/agent/service/MockProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/agent/service/MockProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/agent/service/MockProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/agent/service/MockProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/agent/service/MockProvider/provider/calculate_cost.md)
- [signature_uses_path_key_so_different_edits_do_not_collide](../../../../functions/src/llm/agent/service/signature_uses_path_key_so_different_edits_do_not_collide.md)
- [signature_accepts_file_path_alias](../../../../functions/src/llm/agent/service/signature_accepts_file_path_alias.md)
- [signature_distinguishes_same_tool_different_args](../../../../functions/src/llm/agent/service/signature_distinguishes_same_tool_different_args.md)
- [plan_completion_gate_decision_matrix](../../../../functions/src/llm/agent/service/plan_completion_gate_decision_matrix.md)
- [response_with](../../../../functions/src/llm/agent/service/response_with.md)
- [final_text_falls_back_to_thinking_when_there_is_no_visible_text](../../../../functions/src/llm/agent/service/final_text_falls_back_to_thinking_when_there_is_no_visible_text.md)
- [final_text_prefers_visible_text_and_keeps_thinking_separate](../../../../functions/src/llm/agent/service/final_text_prefers_visible_text_and_keeps_thinking_separate.md)
- [final_text_of_an_empty_response_is_empty](../../../../functions/src/llm/agent/service/final_text_of_an_empty_response_is_empty.md)
- [create_test_service](../../../../functions/src/llm/agent/service/create_test_service.md)
- [test_agent_service_creation](../../../../functions/src/llm/agent/service/test_agent_service_creation.md)
- [test_send_message](../../../../functions/src/llm/agent/service/test_send_message.md)
- [test_send_message_with_system_prompt](../../../../functions/src/llm/agent/service/test_send_message_with_system_prompt.md)
- [system_prompt_tells_the_model_the_working_directory](../../../../functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory.md)
- [system_prompt_with_env_is_none_when_no_prompt_is_set](../../../../functions/src/llm/agent/service/system_prompt_with_env_is_none_when_no_prompt_is_set.md)
- [MockProviderWithTools](../../../../classes/src/llm/agent/service/MockProviderWithTools.md)
- [new](../../../../functions/src/llm/agent/service/MockProviderWithTools/new.md)
- [complete](../../../../functions/src/llm/agent/service/MockProviderWithTools/provider/complete.md)
- [stream](../../../../functions/src/llm/agent/service/MockProviderWithTools/provider/stream.md)
- [name](../../../../functions/src/llm/agent/service/MockProviderWithTools/provider/name.md)
- [default_model](../../../../functions/src/llm/agent/service/MockProviderWithTools/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/agent/service/MockProviderWithTools/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/agent/service/MockProviderWithTools/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/agent/service/MockProviderWithTools/provider/calculate_cost.md)
- [MockTool](../../../../classes/src/llm/agent/service/MockTool.md)
- [name](../../../../functions/src/llm/agent/service/MockTool/crate-llm-tools-tool/name.md)
- [description](../../../../functions/src/llm/agent/service/MockTool/crate-llm-tools-tool/description.md)
- [input_schema](../../../../functions/src/llm/agent/service/MockTool/crate-llm-tools-tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/agent/service/MockTool/crate-llm-tools-tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/agent/service/MockTool/crate-llm-tools-tool/requires_approval.md)
- [execute](../../../../functions/src/llm/agent/service/MockTool/crate-llm-tools-tool/execute.md)
- [test_send_message_with_tool_execution](../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [loop_detection_recovery_message_logic](../../../../functions/src/llm/agent/service/loop_detection_recovery_message_logic.md)
- [streamed_ollama_tool_call_survives_drain](../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [drain_stream_to_response_carries_perf_metrics_through](../../../../functions/src/llm/agent/service/drain_stream_to_response_carries_perf_metrics_through.md)
- [drain_stream_assembles_anthropic_tool_input_from_json_deltas](../../../../functions/src/llm/agent/service/drain_stream_assembles_anthropic_tool_input_from_json_deltas.md)
- [ScriptedProvider](../../../../classes/src/llm/agent/service/ScriptedProvider.md)
- [new](../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [named](../../../../functions/src/llm/agent/service/ScriptedProvider/named.md)
- [requests](../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)
- [complete](../../../../functions/src/llm/agent/service/ScriptedProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/agent/service/ScriptedProvider/provider/stream.md)
- [name](../../../../functions/src/llm/agent/service/ScriptedProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/agent/service/ScriptedProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/agent/service/ScriptedProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/agent/service/ScriptedProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/agent/service/ScriptedProvider/provider/calculate_cost.md)
- [text_reply](../../../../functions/src/llm/agent/service/text_reply.md)
- [tool_call](../../../../functions/src/llm/agent/service/tool_call.md)
- [NamedTool](../../../../classes/src/llm/agent/service/NamedTool.md)
- [name](../../../../functions/src/llm/agent/service/NamedTool/crate-llm-tools-tool/name.md)
- [description](../../../../functions/src/llm/agent/service/NamedTool/crate-llm-tools-tool/description.md)
- [input_schema](../../../../functions/src/llm/agent/service/NamedTool/crate-llm-tools-tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/agent/service/NamedTool/crate-llm-tools-tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/agent/service/NamedTool/crate-llm-tools-tool/requires_approval.md)
- [execute](../../../../functions/src/llm/agent/service/NamedTool/crate-llm-tools-tool/execute.md)
- [registry_of](../../../../functions/src/llm/agent/service/registry_of.md)
- [session_for](../../../../functions/src/llm/agent/service/session_for.md)
- [test_context](../../../../functions/src/llm/agent/service/test_context.md)
- [tool_names](../../../../functions/src/llm/agent/service/tool_names.md)
- [auto_profile_sends_core_tools_on_a_small_window](../../../../functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window.md)
- [auto_profile_sends_every_tool_on_a_large_window](../../../../functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window.md)
- [approval_callback_sees_the_canonical_tool_name](../../../../functions/src/llm/agent/service/approval_callback_sees_the_canonical_tool_name.md)
- [context_overflow_compacts_and_retries_the_round](../../../../functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round.md)
- [context_overflow_with_nothing_to_free_is_an_error](../../../../functions/src/llm/agent/service/context_overflow_with_nothing_to_free_is_an_error.md)
- [estimated_context_overflow_is_rejected_before_dispatch](../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_before_dispatch.md)
- [estimated_context_overflow_is_rejected_without_tools](../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_without_tools.md)
- [BigOutputTool](../../../../classes/src/llm/agent/service/BigOutputTool.md)
- [name](../../../../functions/src/llm/agent/service/BigOutputTool/crate-llm-tools-tool/name.md)
- [description](../../../../functions/src/llm/agent/service/BigOutputTool/crate-llm-tools-tool/description.md)
- [input_schema](../../../../functions/src/llm/agent/service/BigOutputTool/crate-llm-tools-tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/agent/service/BigOutputTool/crate-llm-tools-tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/agent/service/BigOutputTool/crate-llm-tools-tool/requires_approval.md)
- [execute](../../../../functions/src/llm/agent/service/BigOutputTool/crate-llm-tools-tool/execute.md)
- [estimated_overflow_tries_deeper_compaction_first](../../../../functions/src/llm/agent/service/estimated_overflow_tries_deeper_compaction_first.md)
- [a_nearly_full_window_is_cut_deeper_before_sending](../../../../functions/src/llm/agent/service/a_nearly_full_window_is_cut_deeper_before_sending.md)
- [make_room_elides_old_tool_output_when_the_budget_is_tight](../../../../functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight.md)
- [calibration_learns_from_the_provider_and_resets_on_switch](../../../../functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch.md)
- [last_user_text](../../../../functions/src/llm/agent/service/last_user_text.md)
- [round_limit_ends_with_an_answer_instead_of_an_error](../../../../functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error.md)
- [stopping_mid_round_answers_every_pending_tool_call](../../../../functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call.md)
- [WritingTool](../../../../classes/src/llm/agent/service/WritingTool.md)
- [name](../../../../functions/src/llm/agent/service/WritingTool/crate-llm-tools-tool/name.md)
- [description](../../../../functions/src/llm/agent/service/WritingTool/crate-llm-tools-tool/description.md)
- [input_schema](../../../../functions/src/llm/agent/service/WritingTool/crate-llm-tools-tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/agent/service/WritingTool/crate-llm-tools-tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/agent/service/WritingTool/crate-llm-tools-tool/requires_approval.md)
- [execute](../../../../functions/src/llm/agent/service/WritingTool/crate-llm-tools-tool/execute.md)
- [a_cancelled_request_stops_with_cancelled](../../../../functions/src/llm/agent/service/a_cancelled_request_stops_with_cancelled.md)
- [cancelling_during_an_approval_prompt_ends_the_request](../../../../functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request.md)
- [verification_sends_the_model_back_to_check_its_edit](../../../../functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit.md)
- [verification_is_off_unless_enabled](../../../../functions/src/llm/agent/service/verification_is_off_unless_enabled.md)
- [tool_work_is_remembered_across_requests](../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [project_notes_are_added_to_the_system_prompt_when_enabled](../../../../functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled.md)
- [StrictTool](../../../../classes/src/llm/agent/service/StrictTool.md)
- [name](../../../../functions/src/llm/agent/service/StrictTool/crate-llm-tools-tool/name.md)
- [description](../../../../functions/src/llm/agent/service/StrictTool/crate-llm-tools-tool/description.md)
- [input_schema](../../../../functions/src/llm/agent/service/StrictTool/crate-llm-tools-tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/agent/service/StrictTool/crate-llm-tools-tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/agent/service/StrictTool/crate-llm-tools-tool/requires_approval.md)
- [execute](../../../../functions/src/llm/agent/service/StrictTool/crate-llm-tools-tool/execute.md)
- [tool_results](../../../../functions/src/llm/agent/service/tool_results.md)
- [bad_tool_arguments_get_the_expected_arguments_back](../../../../functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back.md)
- [agent_registry](../../../../functions/src/llm/agent/service/agent_registry.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [sub_agents_without_an_approval_prompt_cannot_run_gated_tools](../../../../functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools.md)
- [sub_agent_tool_calls_are_approved_through_the_parent_s_prompt](../../../../functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt.md)
- [task](../../../../functions/src/llm/agent/service/task.md)
- [launcher_for](../../../../functions/src/llm/agent/service/launcher_for.md)
- [a_background_sub_agent_is_collected_with_its_report](../../../../functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report.md)
- [an_unknown_profile_fails_with_the_available_ones](../../../../functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones.md)
- [GatedProvider](../../../../classes/src/llm/agent/service/GatedProvider.md)
- [complete](../../../../functions/src/llm/agent/service/GatedProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/agent/service/GatedProvider/provider/stream.md)
- [name](../../../../functions/src/llm/agent/service/GatedProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/agent/service/GatedProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/agent/service/GatedProvider/provider/supported_models.md)
- [context_window](../../../../functions/src/llm/agent/service/GatedProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/agent/service/GatedProvider/provider/calculate_cost.md)
- [wait_for_status](../../../../functions/src/llm/agent/service/wait_for_status.md)
- [sub_agents_run_one_at_a_time_by_default](../../../../functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default.md)
- [a_running_sub_agent_can_be_cancelled](../../../../functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled.md)
- [fallback_to](../../../../functions/src/llm/agent/service/fallback_to.md)
- [a_failing_provider_falls_back_and_says_so](../../../../functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so.md)
- [an_overflow_falls_back_only_when_allowed](../../../../functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed.md)
- [an_estimated_overflow_reaches_a_fallback_with_an_unknown_window](../../../../functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window.md)
- [errors_that_a_retry_cannot_fix_do_not_fall_back](../../../../functions/src/llm/agent/service/errors_that_a_retry_cannot_fix_do_not_fall_back.md)
- [the_router_picks_the_model_for_the_request](../../../../functions/src/llm/agent/service/the_router_picks_the_model_for_the_request.md)
- [memory_test_store](../../../../functions/src/llm/agent/service/memory_test_store.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [without_memory_the_user_turn_is_unchanged](../../../../functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [a_refused_global_save_stores_nothing](../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)

# Imports

- `super::budget::{
    calibration_from, tool_schema_tokens, ContextBudget, ContextUsage, DEFAULT_CONTEXT_WINDOW,
}`
- `super::compaction`
- `super::context::{token_count, AgentContext}`
- `super::error::{AgentError, Result}`
- `super::middleware::tool_call_signature`
- `super::middleware::{
    has_mutating_capability, Limits, LoopGuard, Pipeline, ToolCallRecord, Verification,
}`
- `super::tool_profile::ToolSelection`
- `crate::config::ToolProfile`
- `crate::llm::provider::router::ModelRouter`
- `crate::llm::provider::{
    ContentBlock, ContentDelta, LLMRequest, LLMResponse, Message, PerfMetrics, Provider,
    ProviderStream, Role, StopReason, StreamEvent, TokenUsage, Tool,
}`
- `crate::llm::tools::cache::{ToolResultCache, ToolTtlConfig}`
- `crate::llm::tools::{FileReadCache, ToolExecutionContext, ToolRegistry}`
- `crate::memory::{MemoryStore, RecallRequest}`
- `crate::services::{MessageService, ServiceContext, SessionService}`
- `futures::StreamExt as _`
- `serde_json::Value`
- `std::future::Future`
- `std::pin::Pin`
- `std::sync::Arc`
- `std::time::Duration`
- `tokio::sync::mpsc`
- `tokio_util::sync::CancellationToken`
- `uuid::Uuid`
- `crate::llm::provider::ProviderError`
- `super::*`
- `crate::db::Database`
- `crate::llm::provider::{LLMRequest, LLMResponse, TokenUsage}`
- `crate::llm::tools::ToolCapability`
- `async_trait::async_trait`
- `crate::llm::provider::{OllamaProvider, Provider, Tool}`
- `crate::llm::provider::types::MessageDelta`
- `crate::llm::provider::ContentDelta`
- `crate::llm::tools::{SubAgentLauncher, SubAgentStatus}`
- `crate::llm::tools::SubAgentLauncher`
- `crate::memory::model::MemoryScope`
- `crate::memory::model::{MemoryCategory, MemorySource, NewMemory}`
- `crate::llm::tools::forget_memory::ForgetMemoryTool`
- `crate::memory::model::{
            MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome,
        }`
- `crate::llm::tools::save_memory::SaveMemoryTool`

# Member of

- [crustly](../../../../packages/crustly.md)