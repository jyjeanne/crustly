---
type: Rust Method
title: show_error
resource: src/tui/app.rs#L3163-L3170
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/execute_next_plan_task
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn show_error(&mut self, error: String)`

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [execute_next_plan_task](../../../../../functions/src/tui/app/App/execute_next_plan_task.md)