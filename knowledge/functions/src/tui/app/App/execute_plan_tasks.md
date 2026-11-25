---
type: Rust Method
title: execute_plan_tasks
resource: src/tui/app.rs#L3021-L3024
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/execute_next_plan_task
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_plan_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute_plan_tasks(&mut self) -> Result<()>`

# Calls

- [execute_next_plan_task](../../../../../functions/src/tui/app/App/execute_next_plan_task.md)

# Called by

- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)