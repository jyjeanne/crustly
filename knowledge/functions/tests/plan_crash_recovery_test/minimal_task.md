---
type: Rust Function
title: minimal_task
resource: tests/plan_crash_recovery_test.rs#L45-L63
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task
  - functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
  - functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
---

# Signature

`fn minimal_task(plan_id: Uuid, task_order: i32, status: &str) -> crustly::db::models::PlanTask`

# Called by

- [crash_recovery_resumes_at_correct_task](../../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)
- [task_state_transitions_correct_order](../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)