---
type: Rust Function
title: mcp_exposure_follows_each_servers_setting
resource: src/llm/agent/tool_profile.rs#L184-L209
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
  - target: functions/src/llm/agent/tool_profile/ToolSelection/with_mcp_exposure
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn mcp_exposure_follows_each_servers_setting()`

# Calls

- [resolve](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/resolve.md)
- [with_mcp_exposure](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/with_mcp_exposure.md)
- [exposes](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)