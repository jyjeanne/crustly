---
type: Rust Method
title: delete
resource: src/db/repository/plan.rs#L248-L258
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/plan/PlanService/delete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn delete(&self, id: Uuid) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [delete](../../../../../../functions/src/services/plan/PlanService/delete.md)