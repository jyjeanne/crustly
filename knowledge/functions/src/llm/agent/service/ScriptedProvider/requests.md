---
type: Rust Method
title: requests
resource: src/llm/agent/service.rs#L2507-L2509
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round
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
  - target: functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit
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
  - target: functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/the_router_picks_the_model_for_the_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window
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
  - target: functions/src/llm/agent/service/a_cancelled_request_stops_with_cancelled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/verification_is_off_unless_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/errors_that_a_retry_cannot_fix_do_not_fall_back
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn requests(&self) -> Vec<LLMRequest>`

# Called by

- [auto_profile_sends_core_tools_on_a_small_window](../../../../../../functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window.md)
- [context_overflow_compacts_and_retries_the_round](../../../../../../functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round.md)
- [estimated_overflow_tries_deeper_compaction_first](../../../../../../functions/src/llm/agent/service/estimated_overflow_tries_deeper_compaction_first.md)
- [a_nearly_full_window_is_cut_deeper_before_sending](../../../../../../functions/src/llm/agent/service/a_nearly_full_window_is_cut_deeper_before_sending.md)
- [round_limit_ends_with_an_answer_instead_of_an_error](../../../../../../functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error.md)
- [stopping_mid_round_answers_every_pending_tool_call](../../../../../../functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call.md)
- [verification_sends_the_model_back_to_check_its_edit](../../../../../../functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit.md)
- [tool_work_is_remembered_across_requests](../../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [project_notes_are_added_to_the_system_prompt_when_enabled](../../../../../../functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled.md)
- [bad_tool_arguments_get_the_expected_arguments_back](../../../../../../functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [sub_agents_without_an_approval_prompt_cannot_run_gated_tools](../../../../../../functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools.md)
- [a_failing_provider_falls_back_and_says_so](../../../../../../functions/src/llm/agent/service/a_failing_provider_falls_back_and_says_so.md)
- [the_router_picks_the_model_for_the_request](../../../../../../functions/src/llm/agent/service/the_router_picks_the_model_for_the_request.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [without_memory_the_user_turn_is_unchanged](../../../../../../functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged.md)
- [auto_profile_sends_every_tool_on_a_large_window](../../../../../../functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window.md)
- [context_overflow_with_nothing_to_free_is_an_error](../../../../../../functions/src/llm/agent/service/context_overflow_with_nothing_to_free_is_an_error.md)
- [estimated_context_overflow_is_rejected_before_dispatch](../../../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_before_dispatch.md)
- [estimated_context_overflow_is_rejected_without_tools](../../../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_without_tools.md)
- [a_cancelled_request_stops_with_cancelled](../../../../../../functions/src/llm/agent/service/a_cancelled_request_stops_with_cancelled.md)
- [verification_is_off_unless_enabled](../../../../../../functions/src/llm/agent/service/verification_is_off_unless_enabled.md)
- [sub_agent_tool_calls_are_approved_through_the_parent_s_prompt](../../../../../../functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt.md)
- [an_unknown_profile_fails_with_the_available_ones](../../../../../../functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones.md)
- [an_overflow_falls_back_only_when_allowed](../../../../../../functions/src/llm/agent/service/an_overflow_falls_back_only_when_allowed.md)
- [an_estimated_overflow_reaches_a_fallback_with_an_unknown_window](../../../../../../functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window.md)
- [errors_that_a_retry_cannot_fix_do_not_fall_back](../../../../../../functions/src/llm/agent/service/errors_that_a_retry_cannot_fix_do_not_fall_back.md)