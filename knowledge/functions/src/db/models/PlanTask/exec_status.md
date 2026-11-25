---
type: Rust Method
title: exec_status
resource: src/db/models.rs#L186-L188
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/db/models/interrupted_plan_from_tasks
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

`pub fn exec_status(&self) -> PlanTaskStatus`

# Calls

- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [interrupted_plan_from_tasks](../../../../../functions/src/db/models/interrupted_plan_from_tasks.md)
- [task_state_transitions_correct_order](../../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)