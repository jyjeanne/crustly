---
type: Rust Function
title: test_topological_sort_no_dependencies
resource: src/plan/plan_tests.rs#L210-L226
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/plan_tests/create_test_task
  - functions/src/plan/PlanDocument/add_task
  - functions/src/plan/PlanDocument/tasks_in_order
---

# Signature

`fn test_topological_sort_no_dependencies()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [tasks_in_order](../../../../functions/src/plan/PlanDocument/tasks_in_order.md)