---
type: Rust Function
title: an_allow_list_exposes_exactly_its_tools
resource: src/llm/agent/tool_profile.rs#L136-L146
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/tool_profile/ToolSelection/only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/is_core_only
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn an_allow_list_exposes_exactly_its_tools()`

# Calls

- [only](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/only.md)
- [exposes](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)
- [is_core_only](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/is_core_only.md)