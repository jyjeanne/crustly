---
type: Rust Function
title: test_get_task_mut
resource: src/plan/plan_tests.rs#L77-L91
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/plan_tests/create_test_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task_mut
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_get_task_mut()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [get_task_mut](../../../../functions/src/plan/PlanDocument/get_task_mut.md)