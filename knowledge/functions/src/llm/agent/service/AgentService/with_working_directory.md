---
type: Rust Method
title: with_working_directory
resource: src/llm/agent/service.rs#L756-L759
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/verification_is_off_unless_enabled
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_work_is_remembered_across_requests
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_working_directory(mut self, working_directory: std::path::PathBuf) -> Self`

# Called by

- [cmd_chat](../../../../../../functions/src/cli/cmd_chat.md)
- [verification_sends_the_model_back_to_check_its_edit](../../../../../../functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit.md)
- [verification_is_off_unless_enabled](../../../../../../functions/src/llm/agent/service/verification_is_off_unless_enabled.md)
- [tool_work_is_remembered_across_requests](../../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [project_notes_are_added_to_the_system_prompt_when_enabled](../../../../../../functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [a_background_sub_agent_is_collected_with_its_report](../../../../../../functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report.md)
- [an_unknown_profile_fails_with_the_available_ones](../../../../../../functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones.md)
- [sub_agents_run_one_at_a_time_by_default](../../../../../../functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default.md)
- [a_running_sub_agent_can_be_cancelled](../../../../../../functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [without_memory_the_user_turn_is_unchanged](../../../../../../functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)