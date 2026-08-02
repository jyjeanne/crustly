---
type: Rust Method
title: create_task
resource: src/db/repository/plan.rs#L492-L514
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task
  - functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
  - functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
---

# Signature

`pub async fn create_task(&self, task: PlanTask) -> Result<()>`

# Called by

- [crash_recovery_resumes_at_correct_task](../../../../../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)
- [task_state_transitions_correct_order](../../../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)