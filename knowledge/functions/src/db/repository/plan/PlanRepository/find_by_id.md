---
type: Rust Method
title: find_by_id
resource: src/db/repository/plan.rs#L31-L47
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id
  - functions/src/db/repository/plan/PlanRepository/plan_from_db
---

# Signature

`pub async fn find_by_id(&self, id: Uuid) -> Result<Option<PlanDocument>>`

# Calls

- [find_tasks_by_plan_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id.md)
- [plan_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)