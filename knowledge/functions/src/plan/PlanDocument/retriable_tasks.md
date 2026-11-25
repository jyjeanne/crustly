---
type: Rust Method
title: retriable_tasks
resource: src/plan/mod.rs#L350-L352
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanTask/can_retry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn retriable_tasks(&self) -> Vec<&PlanTask>`

# Calls

- [can_retry](../../../../functions/src/plan/PlanTask/can_retry.md)