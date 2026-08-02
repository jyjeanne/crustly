---
type: Rust Method
title: find_tasks_by_plan_id
resource: src/db/repository/plan.rs#L69-L79
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/db/repository/plan/PlanRepository/find_by_id
  - functions/src/db/repository/plan/PlanRepository/find_by_session_id
---

# Signature

`async fn find_tasks_by_plan_id(&self, plan_id: Uuid) -> Result<Vec<PlanTask>>`

# Called by

- [find_by_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_by_id.md)
- [find_by_session_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_by_session_id.md)