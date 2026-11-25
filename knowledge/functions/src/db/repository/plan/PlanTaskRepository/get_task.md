---
type: Rust Method
title: get_task
resource: src/db/repository/plan.rs#L558-L586
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/row_to_plan_task
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn get_task(&self, task_id: Uuid) -> Result<PlanTask>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)
- [row_to_plan_task](../../../../../../functions/src/db/repository/plan/row_to_plan_task.md)

# Called by

- [task_state_transitions_correct_order](../../../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)