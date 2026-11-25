---
type: Rust Method
title: overhead_is_tight
resource: src/llm/agent/budget.rs#L205-L208
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/context_usage_color
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn overhead_is_tight(&self) -> bool`

# Called by

- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [full_tool_set_leaves_little_of_an_8k_window](../../../../../../functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window.md)
- [context_usage_color](../../../../../../functions/src/tui/render/context_usage_color.md)