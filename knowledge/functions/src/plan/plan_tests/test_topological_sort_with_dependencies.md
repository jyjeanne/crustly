---
type: Rust Function
title: test_topological_sort_with_dependencies
resource: src/plan/plan_tests.rs#L229-L254
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/plan/plan_tests/create_test_task
  - functions/src/plan/PlanDocument/add_task
  - functions/src/plan/PlanDocument/tasks_in_order
---

# Signature

`fn test_topological_sort_with_dependencies()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [tasks_in_order](../../../../functions/src/plan/PlanDocument/tasks_in_order.md)