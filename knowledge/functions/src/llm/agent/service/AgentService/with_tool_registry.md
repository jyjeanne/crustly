---
type: Rust Method
title: with_tool_registry
resource: src/llm/agent/service.rs#L738-L741
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/test_send_message_with_tool_execution
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
  - target: functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request
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
  - target: functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_refused_global_save_stores_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/create_error_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/create_test_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_tool_registry(mut self, registry: Arc<ToolRegistry>) -> Self`

# Called by

- [cmd_chat](../../../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [test_send_message_with_tool_execution](../../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [auto_profile_sends_core_tools_on_a_small_window](../../../../../../functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window.md)
- [auto_profile_sends_every_tool_on_a_large_window](../../../../../../functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window.md)
- [approval_callback_sees_the_canonical_tool_name](../../../../../../functions/src/llm/agent/service/approval_callback_sees_the_canonical_tool_name.md)
- [estimated_overflow_tries_deeper_compaction_first](../../../../../../functions/src/llm/agent/service/estimated_overflow_tries_deeper_compaction_first.md)
- [a_nearly_full_window_is_cut_deeper_before_sending](../../../../../../functions/src/llm/agent/service/a_nearly_full_window_is_cut_deeper_before_sending.md)
- [round_limit_ends_with_an_answer_instead_of_an_error](../../../../../../functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error.md)
- [stopping_mid_round_answers_every_pending_tool_call](../../../../../../functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call.md)
- [cancelling_during_an_approval_prompt_ends_the_request](../../../../../../functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request.md)
- [verification_sends_the_model_back_to_check_its_edit](../../../../../../functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit.md)
- [verification_is_off_unless_enabled](../../../../../../functions/src/llm/agent/service/verification_is_off_unless_enabled.md)
- [tool_work_is_remembered_across_requests](../../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [bad_tool_arguments_get_the_expected_arguments_back](../../../../../../functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [sub_agents_without_an_approval_prompt_cannot_run_gated_tools](../../../../../../functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools.md)
- [sub_agent_tool_calls_are_approved_through_the_parent_s_prompt](../../../../../../functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [a_refused_global_save_stores_nothing](../../../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
- [create_error_agent](../../../../../../functions/tests/error_scenarios_test/create_error_agent.md)
- [create_test_agent](../../../../../../functions/tests/integration_test/create_test_agent.md)