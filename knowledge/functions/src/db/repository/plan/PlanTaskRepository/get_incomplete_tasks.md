---
type: Rust Method
title: get_incomplete_tasks
resource: src/db/repository/plan.rs#L620-L650
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/plan/PlanService/get_incomplete_tasks
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn get_incomplete_tasks(&self, plan_id: Uuid) -> Result<Vec<PlanTask>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [get_incomplete_tasks](../../../../../../functions/src/services/plan/PlanService/get_incomplete_tasks.md)
- [crash_recovery_resumes_at_correct_task](../../../../../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)