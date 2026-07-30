---
type: Rust Method
title: plan_from_db
resource: src/db/repository/plan.rs#L261-L289
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/db/repository/plan/PlanRepository/parse_plan_status
  - functions/src/db/repository/plan/PlanRepository/task_from_db
  called_by:
  - functions/src/db/repository/plan/PlanRepository/find_by_id
  - functions/src/db/repository/plan/PlanRepository/find_by_session_id
---

# Signature

`fn plan_from_db(&self, db_plan: Plan, db_tasks: Vec<PlanTask>) -> Result<PlanDocument>`

# Calls

- [from_str](../../../../../../functions/src/config/secrets/SecretString/from_str.md)
- [parse_plan_status](../../../../../../functions/src/db/repository/plan/PlanRepository/parse_plan_status.md)
- [task_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/task_from_db.md)

# Called by

- [find_by_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_by_id.md)
- [find_by_session_id](../../../../../../functions/src/db/repository/plan/PlanRepository/find_by_session_id.md)