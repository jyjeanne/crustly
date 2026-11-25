---
type: Rust Method
title: with_tool_profile
resource: src/llm/agent/service.rs#L700-L704
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/the_router_picks_the_model_for_the_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_tool_profile(mut self, profile: ToolProfile, extra: Vec<String>) -> Self`

# Called by

- [configure_agent_loop](../../../../../../functions/src/cli/configure_agent_loop.md)
- [auto_profile_sends_core_tools_on_a_small_window](../../../../../../functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window.md)
- [auto_profile_sends_every_tool_on_a_large_window](../../../../../../functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window.md)
- [the_router_picks_the_model_for_the_request](../../../../../../functions/src/llm/agent/service/the_router_picks_the_model_for_the_request.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)