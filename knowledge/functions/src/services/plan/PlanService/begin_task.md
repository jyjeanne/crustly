---
type: Rust Method
title: begin_task
resource: src/services/plan.rs#L60-L64
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/repository/plan/PlanTaskRepository/update_task_status
---

# Signature

`pub async fn begin_task(&self, task_id: Uuid) -> Result<()>`

# Calls

- [update_task_status](../../../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)