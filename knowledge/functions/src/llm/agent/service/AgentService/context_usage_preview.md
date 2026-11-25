---
type: Rust Method
title: context_usage_preview
resource: src/llm/agent/service.rs#L886-L893
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/default_context_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/tool_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_definitions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/system_prompt_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/measure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/calibration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/is_core_only
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/refresh_context_usage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn context_usage_preview(&self) -> ContextUsage`

# Calls

- [default_context_window](../../../../../../functions/src/llm/agent/service/AgentService/default_context_window.md)
- [tool_selection](../../../../../../functions/src/llm/agent/service/AgentService/tool_selection.md)
- [exposed_tool_definitions](../../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_definitions.md)
- [system_prompt_for](../../../../../../functions/src/llm/agent/service/AgentService/system_prompt_for.md)
- [measure](../../../../../../functions/src/llm/agent/budget/ContextBudget/measure.md)
- [calibration](../../../../../../functions/src/llm/agent/service/AgentService/calibration.md)
- [usage](../../../../../../functions/src/llm/agent/budget/ContextBudget/usage.md)
- [is_core_only](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/is_core_only.md)

# Called by

- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [new](../../../../../../functions/src/tui/app/App/new.md)
- [refresh_context_usage](../../../../../../functions/src/tui/app/App/refresh_context_usage.md)