---
type: Rust Method
title: list_tasks_for_plan
resource: src/db/repository/plan.rs#L589-L617
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_tasks_for_plan(&self, plan_id: Uuid) -> Result<Vec<PlanTask>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)