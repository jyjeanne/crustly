---
type: Rust Function
title: create_test_task
resource: src/plan/plan_tests.rs#L20-L27
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanTask/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/plan/plan_tests/test_add_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_get_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_get_task_mut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_count_by_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_progress_percentage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_topological_sort_no_dependencies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_topological_sort_with_dependencies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_topological_sort_circular_dependency
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_validate_dependencies_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_validate_dependencies_circular
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_task_state_transitions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_task_failure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_task_blocking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_task_skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_complex_dependency_chain
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_test_task(order: usize, title: &str) -> PlanTask`

# Calls

- [new](../../../../functions/src/plan/PlanTask/new.md)

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