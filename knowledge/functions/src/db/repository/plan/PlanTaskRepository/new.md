---
type: Rust Method
title: new
resource: src/db/repository/plan.rs#L487-L489
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/services/plan/PlanService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: SqlitePool) -> Self`

# Called by

- [new](../../../../../../functions/src/services/plan/PlanService/new.md)
- [crash_recovery_resumes_at_correct_task](../../../../../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)
- [task_state_transitions_correct_order](../../../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)