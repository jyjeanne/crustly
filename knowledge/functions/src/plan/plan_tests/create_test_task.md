---
type: Rust Function
title: create_test_task
resource: src/plan/plan_tests.rs#L20-L27
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/plan/plan_tests/test_add_task
  - functions/src/plan/plan_tests/test_get_task
  - functions/src/plan/plan_tests/test_get_task_mut
  - functions/src/plan/plan_tests/test_count_by_status
  - functions/src/plan/plan_tests/test_progress_percentage
  - functions/src/plan/plan_tests/test_is_complete
  - functions/src/plan/plan_tests/test_topological_sort_no_dependencies
  - functions/src/plan/plan_tests/test_topological_sort_with_dependencies
  - functions/src/plan/plan_tests/test_topological_sort_circular_dependency
  - functions/src/plan/plan_tests/test_validate_dependencies_success
  - functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference
  - functions/src/plan/plan_tests/test_validate_dependencies_circular
  - functions/src/plan/plan_tests/test_task_state_transitions
  - functions/src/plan/plan_tests/test_task_failure
  - functions/src/plan/plan_tests/test_task_blocking
  - functions/src/plan/plan_tests/test_task_skip
  - functions/src/plan/plan_tests/test_complex_dependency_chain
---

# Signature

`fn create_test_task(order: usize, title: &str) -> PlanTask`

# Called by

- [test_add_task](../../../../functions/src/plan/plan_tests/test_add_task.md)
- [test_get_task](../../../../functions/src/plan/plan_tests/test_get_task.md)
- [test_get_task_mut](../../../../functions/src/plan/plan_tests/test_get_task_mut.md)
- [test_count_by_status](../../../../functions/src/plan/plan_tests/test_count_by_status.md)
- [test_progress_percentage](../../../../functions/src/plan/plan_tests/test_progress_percentage.md)
- [test_is_complete](../../../../functions/src/plan/plan_tests/test_is_complete.md)
- [test_topological_sort_no_dependencies](../../../../functions/src/plan/plan_tests/test_topological_sort_no_dependencies.md)
- [test_topological_sort_with_dependencies](../../../../functions/src/plan/plan_tests/test_topological_sort_with_dependencies.md)
- [test_topological_sort_circular_dependency](../../../../functions/src/plan/plan_tests/test_topological_sort_circular_dependency.md)
- [test_validate_dependencies_success](../../../../functions/src/plan/plan_tests/test_validate_dependencies_success.md)
- [test_validate_dependencies_invalid_reference](../../../../functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference.md)
- [test_validate_dependencies_circular](../../../../functions/src/plan/plan_tests/test_validate_dependencies_circular.md)
- [test_task_state_transitions](../../../../functions/src/plan/plan_tests/test_task_state_transitions.md)
- [test_task_failure](../../../../functions/src/plan/plan_tests/test_task_failure.md)
- [test_task_blocking](../../../../functions/src/plan/plan_tests/test_task_blocking.md)
- [test_task_skip](../../../../functions/src/plan/plan_tests/test_task_skip.md)
- [test_complex_dependency_chain](../../../../functions/src/plan/plan_tests/test_complex_dependency_chain.md)