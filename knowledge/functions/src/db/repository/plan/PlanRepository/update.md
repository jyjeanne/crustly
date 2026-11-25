---
type: Rust Method
title: update
resource: src/db/repository/plan.rs#L145-L245
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanRepository/plan_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/plan/PlanService/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn update(&self, plan: &PlanDocument) -> Result<()>`

# Calls

- [plan_to_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_to_db.md)
- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [update](../../../../../../functions/src/services/plan/PlanService/update.md)