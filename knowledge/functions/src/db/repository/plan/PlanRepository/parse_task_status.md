---
type: Rust Method
title: parse_task_status
resource: src/db/repository/plan.rs#L439-L459
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/db/repository/plan/PlanRepository/task_from_db
---

# Signature

`fn parse_task_status(&self, status: &str) -> Result<TaskStatus>`

# Called by

- [task_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/task_from_db.md)