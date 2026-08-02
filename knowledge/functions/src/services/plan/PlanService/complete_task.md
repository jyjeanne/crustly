---
type: Rust Method
title: complete_task
resource: src/services/plan.rs#L69-L73
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/repository/plan/PlanTaskRepository/update_task_status
---

# Signature

`pub async fn complete_task(&self, task_id: Uuid, output_summary: Option<String>) -> Result<()>`

# Calls

- [update_task_status](../../../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)