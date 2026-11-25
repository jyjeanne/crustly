---
type: Rust Method
title: tool_selection
resource: src/llm/agent/service.rs#L861-L867
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/tool_profile/ToolSelection/only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/resolve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/ToolSelection/with_mcp_exposure
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tool_selection(&self, context_window: u32) -> ToolSelection`

# Calls

- [only](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/only.md)
- [resolve](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/resolve.md)
- [with_mcp_exposure](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/with_mcp_exposure.md)

# Called by

- [exposed_tool_names](../../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_names.md)
- [context_usage_preview](../../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)