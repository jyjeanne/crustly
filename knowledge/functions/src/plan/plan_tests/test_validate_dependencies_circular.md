---
type: Rust Function
title: test_validate_dependencies_circular
resource: src/plan/plan_tests.rs#L311-L330
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/plan_tests/create_test_task
  - functions/src/plan/PlanDocument/add_task
  - functions/src/plan/PlanDocument/validate_dependencies
---

# Signature

`fn test_validate_dependencies_circular()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [validate_dependencies](../../../../functions/src/plan/PlanDocument/validate_dependencies.md)