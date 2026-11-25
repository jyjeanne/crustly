---
type: Rust Method
title: parse_plan_status
resource: src/db/repository/plan.rs#L378-L389
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/repository/plan/PlanRepository/plan_from_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_plan_status(&self, status: &str) -> Result<PlanStatus>`

# Called by

- [plan_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)