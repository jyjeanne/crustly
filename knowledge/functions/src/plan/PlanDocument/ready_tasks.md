---
type: Rust Method
title: ready_tasks
resource: src/plan/mod.rs#L340-L347
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/dependencies_satisfied
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ready_tasks(&self) -> Vec<&PlanTask>`

# Calls

- [dependencies_satisfied](../../../../functions/src/plan/PlanDocument/dependencies_satisfied.md)