---
type: Rust Function
title: row_to_plan_task
resource: src/db/repository/plan.rs#L654-L708
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  called_by:
  - functions/src/db/repository/plan/PlanTaskRepository/get_task
---

# Signature

`fn row_to_plan_task( row: ( String, String, i32, String, String, String, String, i32, String, String, Option<String>, Option<i64>, Option<i64>, Option<String>, Option<String>, ), ) -> PlanTask`

# Calls

- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [get_task](../../../../../functions/src/db/repository/plan/PlanTaskRepository/get_task.md)