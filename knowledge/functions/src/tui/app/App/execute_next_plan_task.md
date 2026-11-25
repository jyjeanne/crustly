---
type: Rust Method
title: execute_next_plan_task
resource: src/tui/app.rs#L3027-L3127
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/tasks_in_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/show_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/save_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/send_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/App/complete_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/execute_plan_tasks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute_next_plan_task(&mut self) -> Result<()>`

# Calls

- [tasks_in_order](../../../../../functions/src/plan/PlanDocument/tasks_in_order.md)
- [show_error](../../../../../functions/src/tui/app/App/show_error.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)
- [send_message](../../../../../functions/src/tui/app/App/send_message.md)

# Called by

- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)
- [execute_plan_tasks](../../../../../functions/src/tui/app/App/execute_plan_tasks.md)