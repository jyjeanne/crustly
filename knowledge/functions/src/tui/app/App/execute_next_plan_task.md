---
type: Rust Method
title: execute_next_plan_task
resource: src/tui/app.rs#L1918-L2018
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/plan/PlanDocument/tasks_in_order
  - functions/src/tui/app/App/show_error
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/tools/task/FileLock/drop/drop
  - functions/src/tui/app/App/save_plan
  called_by:
  - functions/src/tui/app/App/complete_response
  - functions/src/tui/app/App/execute_plan_tasks
---

# Signature

`async fn execute_next_plan_task(&mut self) -> Result<()>`

# Calls

- [tasks_in_order](../../../../../functions/src/plan/PlanDocument/tasks_in_order.md)
- [show_error](../../../../../functions/src/tui/app/App/show_error.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [drop](../../../../../functions/src/llm/tools/task/FileLock/drop/drop.md)
- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)

# Called by

- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)
- [execute_plan_tasks](../../../../../functions/src/tui/app/App/execute_plan_tasks.md)