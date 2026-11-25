---
type: Rust Function
title: explicit_profiles_ignore_the_window
resource: src/llm/agent/tool_profile.rs#L147-L150
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`fn explicit_profiles_ignore_the_window()`

# Calls

- [resolve](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/resolve.md)
- [is_core_only](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/is_core_only.md)