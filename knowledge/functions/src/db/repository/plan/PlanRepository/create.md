---
type: Rust Method
title: create
resource: src/db/repository/plan.rs#L82-L142
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
  - target: functions/src/services/plan/PlanService/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn create(&self, plan: &PlanDocument) -> Result<()>`

# Calls

- [plan_to_db](../../../../../../functions/src/db/repository/plan/PlanRepository/plan_to_db.md)
- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [create](../../../../../../functions/src/services/plan/PlanService/create.md)