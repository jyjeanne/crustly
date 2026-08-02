---
type: Rust Method
title: format_task_status
resource: src/db/repository/plan.rs#L462-L471
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/db/repository/plan/PlanRepository/task_to_db
---

# Signature

`fn format_task_status(&self, status: &TaskStatus) -> String`

# Called by

- [task_to_db](../../../../../../functions/src/db/repository/plan/PlanRepository/task_to_db.md)