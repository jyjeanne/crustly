---
type: Rust Method
title: tasks_in_order
resource: src/plan/mod.rs#L86-L145
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/plan/PlanDocument/validate_dependencies
  - functions/src/plan/plan_tests/test_topological_sort_no_dependencies
  - functions/src/plan/plan_tests/test_topological_sort_with_dependencies
  - functions/src/plan/plan_tests/test_topological_sort_circular_dependency
  - functions/src/plan/plan_tests/test_complex_dependency_chain
  - functions/src/tui/app/App/execute_next_plan_task
---

# Signature

`pub fn tasks_in_order(&self) -> Option<Vec<&PlanTask>>`

# Calls

- [len](../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [validate_dependencies](../../../../functions/src/plan/PlanDocument/validate_dependencies.md)
- [test_topological_sort_no_dependencies](../../../../functions/src/plan/plan_tests/test_topological_sort_no_dependencies.md)
- [test_topological_sort_with_dependencies](../../../../functions/src/plan/plan_tests/test_topological_sort_with_dependencies.md)
- [test_topological_sort_circular_dependency](../../../../functions/src/plan/plan_tests/test_topological_sort_circular_dependency.md)
- [test_complex_dependency_chain](../../../../functions/src/plan/plan_tests/test_complex_dependency_chain.md)
- [execute_next_plan_task](../../../../functions/src/tui/app/App/execute_next_plan_task.md)