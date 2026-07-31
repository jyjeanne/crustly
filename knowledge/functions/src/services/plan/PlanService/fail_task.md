---
type: Rust Method
title: fail_task
resource: src/services/plan.rs#L76-L80
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/repository/plan/PlanTaskRepository/update_task_status
---

# Signature

`pub async fn fail_task(&self, task_id: Uuid, error: String) -> Result<()>`

# Calls

- [update_task_status](../../../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)