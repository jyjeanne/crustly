---
type: Rust Method
title: task_from_db
resource: src/db/repository/plan.rs#L292-L320
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/db/repository/plan/PlanRepository/parse_task_type
  - functions/src/db/repository/plan/PlanRepository/parse_task_status
  called_by:
  - functions/src/db/repository/plan/PlanRepository/plan_from_db
---

# Signature

`fn task_from_db(&self, db_task: PlanTask) -> Result<crate::plan::PlanTask>`

# Calls

- [from_str](../../../../../../functions/src/config/secrets/SecretString/from_str.md)
- [parse_task_type](../../../../../../functions/src/db/repository/plan/PlanRepository/parse_task_type.md)
- [parse_task_status](../../../../../../functions/src/db/repository/plan/PlanRepository/parse_task_status.md)

# Called by

- [plan_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)