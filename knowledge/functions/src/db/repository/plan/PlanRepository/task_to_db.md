---
type: Rust Method
title: task_to_db
resource: src/db/repository/plan.rs#L352-L375
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/repository/plan/PlanRepository/format_task_type
  - functions/src/db/repository/plan/PlanRepository/format_task_status
  called_by:
  - functions/src/db/repository/plan/PlanRepository/plan_to_db
---

# Signature

`fn task_to_db(&self, task: &crate::plan::PlanTask, plan_id: Uuid) -> Result<PlanTask>`

# Calls

- [format_task_type](../../../../../../functions/src/db/repository/plan/PlanRepository/format_task_type.md)
- [format_task_status](../../../../../../functions/src/db/repository/plan/PlanRepository/format_task_status.md)

# Called by

- [plan_to_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_to_db.md)