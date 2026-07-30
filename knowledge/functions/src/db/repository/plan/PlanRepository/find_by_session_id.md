---
type: Rust Method
title: find_by_session_id
resource: src/db/repository/plan.rs#L50-L66
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id
  - functions/src/db/repository/plan/PlanRepository/plan_from_db
---

# Signature

`pub async fn find_by_session_id(&self, session_id: Uuid) -> Result<Vec<PlanDocument>>`

# Calls

- [find_tasks_by_plan_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id.md)
- [plan_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)