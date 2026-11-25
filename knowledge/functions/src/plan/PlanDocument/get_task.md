---
type: Rust Method
title: get_task
resource: src/plan/mod.rs#L148-L150
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/plan/PlanDocument/dependencies_satisfied
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/plan_tests/test_get_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn get_task(&self, task_id: &Uuid) -> Option<&PlanTask>`

# Calls

- [find](../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [dependencies_satisfied](../../../../functions/src/plan/PlanDocument/dependencies_satisfied.md)
- [test_get_task](../../../../functions/src/plan/plan_tests/test_get_task.md)