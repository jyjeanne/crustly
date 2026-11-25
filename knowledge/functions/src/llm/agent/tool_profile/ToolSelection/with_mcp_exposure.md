---
type: Rust Method
title: with_mcp_exposure
resource: src/llm/agent/tool_profile.rs#L61-L64
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/tool_selection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/mcp_exposure_follows_each_servers_setting
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_mcp_exposure(mut self, exposure: &[(String, McpExpose)]) -> Self`

# Called by

- [tool_selection](../../../../../../functions/src/llm/agent/service/AgentService/tool_selection.md)
- [mcp_exposure_follows_each_servers_setting](../../../../../../functions/src/llm/agent/tool_profile/mcp_exposure_follows_each_servers_setting.md)