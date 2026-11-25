---
type: Rust Function
title: tool_results
resource: src/llm/agent/service.rs#L3515-L3525
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
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
---

# Signature

`fn tool_results(request: &LLMRequest) -> Vec<String>`

# Called by

- [bad_tool_arguments_get_the_expected_arguments_back](../../../../../functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [sub_agents_without_an_approval_prompt_cannot_run_gated_tools](../../../../../functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools.md)
- [sub_agent_tool_calls_are_approved_through_the_parent_s_prompt](../../../../../functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt.md)