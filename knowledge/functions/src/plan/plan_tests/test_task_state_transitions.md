---
type: Rust Function
title: test_task_state_transitions
resource: src/plan/plan_tests.rs#L333-L346
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/plan_tests/create_test_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/start
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/plan/PlanTask/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_task_state_transitions()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [start](../../../../functions/src/plan/PlanTask/start.md)
- [complete](../../../../functions/src/plan/PlanTask/complete.md)