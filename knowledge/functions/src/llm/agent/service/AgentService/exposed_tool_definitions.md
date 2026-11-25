---
type: Rust Method
title: exposed_tool_definitions
resource: src/llm/agent/service.rs#L857-L863
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/get_tool_definitions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
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

`fn exposed_tool_definitions(&self, selection: &ToolSelection) -> Vec<Tool>`

# Calls

- [get_tool_definitions](../../../../../../functions/src/llm/tools/registry/ToolRegistry/get_tool_definitions.md)
- [exposes](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)

# Called by

- [exposed_tool_names](../../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_names.md)
- [context_usage_preview](../../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)