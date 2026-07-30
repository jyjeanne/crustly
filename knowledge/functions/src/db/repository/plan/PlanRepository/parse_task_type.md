---
type: Rust Method
title: parse_task_type
resource: src/db/repository/plan.rs#L406-L419
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/db/repository/plan/PlanRepository/task_from_db
---

# Signature

`fn parse_task_type(&self, task_type: &str) -> Result<TaskType>`

# Called by

- [task_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/task_from_db.md)