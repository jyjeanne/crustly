---
type: Rust Method
title: task_from_db
resource: src/db/repository/plan.rs#L292-L320
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/parse_task_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/parse_task_status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/plan/PlanRepository/plan_from_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn task_from_db(&self, db_task: PlanTask) -> Result<crate::plan::PlanTask>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)
- [parse_task_type](../../../../../../functions/src/db/repository/plan/PlanRepository/parse_task_type.md)
- [parse_task_status](../../../../../../functions/src/db/repository/plan/PlanRepository/parse_task_status.md)

# Called by

- [plan_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)