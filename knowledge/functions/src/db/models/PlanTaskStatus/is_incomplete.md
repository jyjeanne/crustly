---
type: Rust Method
title: is_incomplete
resource: src/db/models.rs#L153-L155
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/models/interrupted_plan_from_tasks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_incomplete(&self) -> bool`

# Called by

- [interrupted_plan_from_tasks](../../../../../functions/src/db/models/interrupted_plan_from_tasks.md)