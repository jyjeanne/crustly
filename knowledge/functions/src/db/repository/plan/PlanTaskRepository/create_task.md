---
type: Rust Method
title: create_task
resource: src/db/repository/plan.rs#L492-L514
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn create_task(&self, task: PlanTask) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [crash_recovery_resumes_at_correct_task](../../../../../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)
- [task_state_transitions_correct_order](../../../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)