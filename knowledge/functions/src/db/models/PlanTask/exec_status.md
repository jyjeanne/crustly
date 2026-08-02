---
type: Rust Method
title: exec_status
resource: src/db/models.rs#L183-L185
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  called_by:
  - functions/src/db/models/interrupted_plan_from_tasks
---

# Signature

`pub fn exec_status(&self) -> PlanTaskStatus`

# Calls

- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [interrupted_plan_from_tasks](../../../../../functions/src/db/models/interrupted_plan_from_tasks.md)