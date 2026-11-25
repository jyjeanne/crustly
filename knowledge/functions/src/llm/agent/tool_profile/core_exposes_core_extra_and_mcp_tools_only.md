---
type: Rust Function
title: core_exposes_core_extra_and_mcp_tools_only
resource: src/llm/agent/tool_profile.rs#L120-L133
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
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn core_exposes_core_extra_and_mcp_tools_only()`

# Calls

- [resolve](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/resolve.md)
- [exposes](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)