---
type: Rust Method
title: get_task
resource: src/db/repository/plan.rs#L558-L586
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/repository/plan/row_to_plan_task
---

# Signature

`pub async fn get_task(&self, task_id: Uuid) -> Result<PlanTask>`

# Calls

- [row_to_plan_task](../../../../../../functions/src/db/repository/plan/row_to_plan_task.md)