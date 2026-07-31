---
type: Rust Method
title: format_plan_status
resource: src/db/repository/plan.rs#L392-L403
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/db/repository/plan/PlanRepository/plan_to_db
---

# Signature

`fn format_plan_status(&self, status: &PlanStatus) -> String`

# Called by

- [plan_to_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_to_db.md)