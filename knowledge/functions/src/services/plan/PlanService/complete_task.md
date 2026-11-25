---
type: Rust Method
title: complete_task
resource: src/services/plan.rs#L69-L73
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/plan/PlanTaskRepository/update_task_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn complete_task(&self, task_id: Uuid, output_summary: Option<String>) -> Result<()>`

# Calls

- [update_task_status](../../../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)