---
type: Rust Method
title: get_incomplete_tasks
resource: src/services/plan.rs#L83-L88
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanTaskRepository/get_incomplete_tasks
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn get_incomplete_tasks( &self, plan_id: Uuid, ) -> Result<Vec<crate::db::models::PlanTask>>`

# Calls

- [get_incomplete_tasks](../../../../../functions/src/db/repository/plan/PlanTaskRepository/get_incomplete_tasks.md)