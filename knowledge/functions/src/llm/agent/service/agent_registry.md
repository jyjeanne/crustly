---
type: Rust Function
title: agent_registry
resource: src/llm/agent/service.rs#L3573-L3583
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
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

`fn agent_registry(tools: &[(&'static str, bool)]) -> Arc<ToolRegistry>`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)

# Called by

- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [sub_agents_without_an_approval_prompt_cannot_run_gated_tools](../../../../../functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools.md)
- [sub_agent_tool_calls_are_approved_through_the_parent_s_prompt](../../../../../functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt.md)