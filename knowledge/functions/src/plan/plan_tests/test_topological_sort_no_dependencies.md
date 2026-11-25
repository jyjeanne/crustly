---
type: Rust Function
title: test_topological_sort_no_dependencies
resource: src/plan/plan_tests.rs#L210-L226
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
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/tasks_in_order
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_topological_sort_no_dependencies()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [tasks_in_order](../../../../functions/src/plan/PlanDocument/tasks_in_order.md)