---
type: Rust Function
title: row_to_plan_task
resource: src/db/repository/plan.rs#L654-L708
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/plan/PlanTaskRepository/get_task
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn row_to_plan_task( row: ( String, String, i32, String, String, String, String, i32, String, String, Option<String>, Option<i64>, Option<i64>, Option<String>, Option<String>, ), ) -> PlanTask`

# Calls

- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [get_task](../../../../../functions/src/db/repository/plan/PlanTaskRepository/get_task.md)