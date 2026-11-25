---
type: Rust Method
title: parse_task_type
resource: src/db/repository/plan.rs#L406-L419
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/repository/plan/PlanRepository/task_from_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_task_type(&self, task_type: &str) -> Result<TaskType>`

# Called by

- [task_from_db](../../../../../../functions/src/db/repository/plan/PlanRepository/task_from_db.md)