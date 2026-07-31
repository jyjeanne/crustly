---
type: Rust Method
title: execute_plan_tasks
resource: src/tui/app.rs#L1912-L1915
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/execute_next_plan_task
  called_by:
  - functions/src/tui/app/App/handle_plan_key
---

# Signature

`async fn execute_plan_tasks(&mut self) -> Result<()>`

# Calls

- [execute_next_plan_task](../../../../../functions/src/tui/app/App/execute_next_plan_task.md)

# Called by

- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)