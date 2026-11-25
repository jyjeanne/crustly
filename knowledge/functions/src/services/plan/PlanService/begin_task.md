---
type: Rust Method
title: begin_task
resource: src/services/plan.rs#L60-L64
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanTaskRepository/update_task_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn begin_task(&self, task_id: Uuid) -> Result<()>`

# Calls

- [update_task_status](../../../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)