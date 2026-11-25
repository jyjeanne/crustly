---
type: Rust Method
title: system_prompt_for
resource: src/llm/agent/service.rs#L781-L798
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/tool_profile/ToolSelection/is_core_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/load_project_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/notes_budget
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn system_prompt_for(&self, selection: &ToolSelection, window: u32) -> Option<String>`

# Calls

- [is_core_only](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/is_core_only.md)
- [with_env](../../../../../../functions/src/llm/agent/service/AgentService/with_env.md)
- [load_project_notes](../../../../../../functions/src/llm/agent/project_memory/load_project_notes.md)
- [notes_budget](../../../../../../functions/src/llm/agent/project_memory/notes_budget.md)

# Called by

- [context_usage_preview](../../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)