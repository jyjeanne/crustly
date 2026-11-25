---
type: Rust Method
title: task_to_db
resource: src/db/repository/plan.rs#L352-L375
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/format_task_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/format_task_status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/plan/PlanRepository/plan_to_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn task_to_db(&self, task: &crate::plan::PlanTask, plan_id: Uuid) -> Result<PlanTask>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)
- [format_task_type](../../../../../../functions/src/db/repository/plan/PlanRepository/format_task_type.md)
- [format_task_status](../../../../../../functions/src/db/repository/plan/PlanRepository/format_task_status.md)

# Called by

- [plan_to_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_to_db.md)