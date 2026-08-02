---
type: Rust Method
title: update_task_status
resource: src/db/repository/plan.rs#L519-L555
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/services/plan/PlanService/begin_task
  - functions/src/services/plan/PlanService/complete_task
  - functions/src/services/plan/PlanService/fail_task
  - functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
  - functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
---

# Signature

`pub async fn update_task_status( &self, task_id: Uuid, status: PlanTaskStatus, output_summary: Option<String>, error_text: Option<String>, ) -> Result<()>`

# Called by

- [begin_task](../../../../../../functions/src/services/plan/PlanService/begin_task.md)
- [complete_task](../../../../../../functions/src/services/plan/PlanService/complete_task.md)
- [fail_task](../../../../../../functions/src/services/plan/PlanService/fail_task.md)
- [task_state_transitions_correct_order](../../../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)