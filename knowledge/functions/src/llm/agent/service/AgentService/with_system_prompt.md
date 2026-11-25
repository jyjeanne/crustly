---
type: Rust Method
title: with_system_prompt
resource: src/llm/agent/service.rs#L726-L729
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/estimated_context_overflow_is_rejected_before_dispatch
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/estimated_context_overflow_is_rejected_without_tools
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/the_router_picks_the_model_for_the_request
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_system_prompt(mut self, prompt: String) -> Self`

# Called by

- [cmd_chat](../../../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [auto_profile_sends_core_tools_on_a_small_window](../../../../../../functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window.md)
- [estimated_context_overflow_is_rejected_before_dispatch](../../../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_before_dispatch.md)
- [estimated_context_overflow_is_rejected_without_tools](../../../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_without_tools.md)
- [project_notes_are_added_to_the_system_prompt_when_enabled](../../../../../../functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [an_estimated_overflow_reaches_a_fallback_with_an_unknown_window](../../../../../../functions/src/llm/agent/service/an_estimated_overflow_reaches_a_fallback_with_an_unknown_window.md)
- [the_router_picks_the_model_for_the_request](../../../../../../functions/src/llm/agent/service/the_router_picks_the_model_for_the_request.md)
- [test_end_to_end_system_prompt](../../../../../../functions/tests/integration_test/test_end_to_end_system_prompt.md)