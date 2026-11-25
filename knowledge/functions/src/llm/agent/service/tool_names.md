---
type: Rust Function
title: tool_names
resource: src/llm/agent/service.rs#L2672-L2681
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tool_names(request: &LLMRequest) -> Vec<String>`

# Called by

- [auto_profile_sends_core_tools_on_a_small_window](../../../../../functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window.md)
- [auto_profile_sends_every_tool_on_a_large_window](../../../../../functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)