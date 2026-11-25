---
type: Rust Method
title: exposed_tool_names
resource: src/llm/agent/service.rs#L885-L894
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/tool_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/default_context_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_definitions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/send_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn exposed_tool_names(&self) -> Vec<String>`

# Calls

- [tool_selection](../../../../../../functions/src/llm/agent/service/AgentService/tool_selection.md)
- [default_context_window](../../../../../../functions/src/llm/agent/service/AgentService/default_context_window.md)
- [exposed_tool_definitions](../../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_definitions.md)

# Called by

- [auto_profile_sends_every_tool_on_a_large_window](../../../../../../functions/src/llm/agent/service/auto_profile_sends_every_tool_on_a_large_window.md)
- [send_message](../../../../../../functions/src/tui/app/App/send_message.md)