---
type: Rust Function
title: interrupted_plan_from_tasks
resource: src/db/models.rs#L198-L219
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTask/exec_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/PlanTaskStatus/is_incomplete
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/plan_crash_recovery_test/interrupted_plan_none_when_all_done
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_crash_recovery_test/interrupted_plan_resumes_at_lowest_incomplete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn interrupted_plan_from_tasks( plan_id: Uuid, tasks: &[PlanTask], ) -> Option<crate::plan::InterruptedPlan>`

# Calls

- [exec_status](../../../../functions/src/db/models/PlanTask/exec_status.md)
- [is_incomplete](../../../../functions/src/db/models/PlanTaskStatus/is_incomplete.md)

# Called by

- [interrupted_plan_none_when_all_done](../../../../functions/tests/plan_crash_recovery_test/interrupted_plan_none_when_all_done.md)
- [interrupted_plan_resumes_at_lowest_incomplete](../../../../functions/tests/plan_crash_recovery_test/interrupted_plan_resumes_at_lowest_incomplete.md)