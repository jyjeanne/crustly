---
type: Rust Function
title: test_complex_dependency_chain
resource: src/plan/plan_tests.rs#L478-L537
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/plan_tests/create_test_task
  - functions/src/plan/PlanDocument/add_task
  - functions/src/plan/PlanDocument/validate_dependencies
  - functions/src/plan/PlanDocument/tasks_in_order
---

# Signature

`fn test_complex_dependency_chain()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [validate_dependencies](../../../../functions/src/plan/PlanDocument/validate_dependencies.md)
- [tasks_in_order](../../../../functions/src/plan/PlanDocument/tasks_in_order.md)