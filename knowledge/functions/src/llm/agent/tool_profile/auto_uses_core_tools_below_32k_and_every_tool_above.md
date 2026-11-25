---
type: Rust Function
title: auto_uses_core_tools_below_32k_and_every_tool_above
resource: src/llm/agent/tool_profile.rs#L106-L111
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/tool_profile/ToolSelection/resolve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/ToolSelection/is_core_only
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn auto_uses_core_tools_below_32k_and_every_tool_above()`

# Calls

- [resolve](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/resolve.md)
- [is_core_only](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/is_core_only.md)