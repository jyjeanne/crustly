---
type: Rust Method
title: update_task_status
resource: src/db/repository/plan.rs#L519-L555
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/plan/PlanService/begin_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/complete_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/fail_task
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

`pub async fn update_task_status( &self, task_id: Uuid, status: PlanTaskStatus, output_summary: Option<String>, error_text: Option<String>, ) -> Result<()>`

# Calls

- [as_str](../../../../../../functions/src/db/models/PlanTaskStatus/as_str.md)
- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [begin_task](../../../../../../functions/src/services/plan/PlanService/begin_task.md)
- [complete_task](../../../../../../functions/src/services/plan/PlanService/complete_task.md)
- [fail_task](../../../../../../functions/src/services/plan/PlanService/fail_task.md)
- [task_state_transitions_correct_order](../../../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)