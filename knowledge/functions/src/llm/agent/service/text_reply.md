---
type: Rust Function
title: text_reply
resource: src/llm/agent/service.rs#L2571-L2586
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/ScriptedProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/GatedProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/approval_callback_sees_the_canonical_tool_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/context_overflow_with_nothing_to_free_is_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/estimated_context_overflow_is_rejected_before_dispatch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/estimated_context_overflow_is_rejected_without_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/estimated_overflow_tries_deeper_compaction_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_nearly_full_window_is_cut_deeper_before_sending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_cancelled_request_stops_with_cancelled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/verification_is_off_unless_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_work_is_remembered_across_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_refused_global_save_stores_nothing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn text_reply(text: &str) -> LLMResponse`

# Called by

- [complete](../../../../../functions/src/llm/agent/service/ScriptedProvider/provider/complete.md)
- [calibration_learns_from_the_provider_and_resets_on_switch](../../../../../functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch.md)
- [complete](../../../../../functions/src/llm/agent/service/GatedProvider/provider/complete.md)
- [auto_profile_sends_core_tools_on_a_small_window](../../../../../functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window.md)
- [auto_profile_sends_every_tool_on_a_large_window](../../../../../functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window.md)
- [approval_callback_sees_the_canonical_tool_name](../../../../../functions/src/llm/agent/service/approval_callback_sees_the_canonical_tool_name.md)
- [context_overflow_compacts_and_retries_the_round](../../../../../functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round.md)
- [context_overflow_with_nothing_to_free_is_an_error](../../../../../functions/src/llm/agent/service/context_overflow_with_nothing_to_free_is_an_error.md)
- [estimated_context_overflow_is_rejected_before_dispatch](../../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_before_dispatch.md)
- [estimated_context_overflow_is_rejected_without_tools](../../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_without_tools.md)
- [estimated_overflow_tries_deeper_compaction_first](../../../../../functions/src/llm/agent/service/estimated_overflow_tries_deeper_compaction_first.md)
- [a_nearly_full_window_is_cut_deeper_before_sending](../../../../../functions/src/llm/agent/service/a_nearly_full_window_is_cut_deeper_before_sending.md)
- [round_limit_ends_with_an_answer_instead_of_an_error](../../../../../functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error.md)
- [stopping_mid_round_answers_every_pending_tool_call](../../../../../functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call.md)
- [a_cancelled_request_stops_with_cancelled](../../../../../functions/src/llm/agent/service/a_cancelled_request_stops_with_cancelled.md)
- [verification_sends_the_model_back_to_check_its_edit](../../../../../functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit.md)
- [verification_is_off_unless_enabled](../../../../../functions/src/llm/agent/service/verification_is_off_unless_enabled.md)
- [tool_work_is_remembered_across_requests](../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [project_notes_are_added_to_the_system_prompt_when_enabled](../../../../../functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled.md)
- [bad_tool_arguments_get_the_expected_arguments_back](../../../../../functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [sub_agents_without_an_approval_prompt_cannot_run_gated_tools](../../../../../functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools.md)
- [sub_agent_tool_calls_are_approved_through_the_parent_s_prompt](../../../../../functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt.md)
- [a_background_sub_agent_is_collected_with_its_report](../../../../../functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report.md)
- [a_failing_provider_falls_back_and_says_so](../../../../../functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so.md)
- [an_overflow_falls_back_only_when_allowed](../../../../../functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed.md)
- [an_estimated_overflow_reaches_a_fallback_with_an_unknown_window](../../../../../functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [without_memory_the_user_turn_is_unchanged](../../../../../functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [a_refused_global_save_stores_nothing](../../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)