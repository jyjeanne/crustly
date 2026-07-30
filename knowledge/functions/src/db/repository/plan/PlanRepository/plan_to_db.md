---
type: Rust Method
title: plan_to_db
resource: src/db/repository/plan.rs#L323-L349
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/repository/plan/PlanRepository/format_plan_status
  - functions/src/db/repository/plan/PlanRepository/task_to_db
  called_by:
  - functions/src/db/repository/plan/PlanRepository/create
  - functions/src/db/repository/plan/PlanRepository/update
---

# Signature

`fn plan_to_db(&self, plan: &PlanDocument) -> Result<(Plan, Vec<PlanTask>)>`

# Calls

- [format_plan_status](../../../../../../functions/src/db/repository/plan/PlanRepository/format_plan_status.md)
- [task_to_db](../../../../../../functions/src/db/repository/plan/PlanRepository/task_to_db.md)

# Called by

- [create](../../../../../../functions/src/db/repository/plan/PlanRepository/create.md)
- [update](../../../../../../functions/src/db/repository/plan/PlanRepository/update.md)