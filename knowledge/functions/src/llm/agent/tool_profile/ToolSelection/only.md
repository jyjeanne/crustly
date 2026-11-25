---
type: Rust Method
title: only
resource: src/llm/agent/tool_profile.rs#L50-L57
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/tool_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/an_allow_list_exposes_exactly_its_tools
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn only(names: &[String]) -> Self`

# Called by

- [tool_selection](../../../../../../functions/src/llm/agent/service/AgentService/tool_selection.md)
- [an_allow_list_exposes_exactly_its_tools](../../../../../../functions/src/llm/agent/tool_profile/an_allow_list_exposes_exactly_its_tools.md)