---
type: Rust Module
title: plan_tests
resource: src/plan/plan_tests.rs#L1-L568
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crate-plan
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [create_test_plan](../../../functions/src/plan/plan_tests/create_test_plan.md)
- [create_test_task](../../../functions/src/plan/plan_tests/create_test_task.md)
- [test_plan_document_new](../../../functions/src/plan/plan_tests/test_plan_document_new.md)
- [test_add_task](../../../functions/src/plan/plan_tests/test_add_task.md)
- [test_get_task](../../../functions/src/plan/plan_tests/test_get_task.md)
- [test_get_task_mut](../../../functions/src/plan/plan_tests/test_get_task_mut.md)
- [test_count_by_status](../../../functions/src/plan/plan_tests/test_count_by_status.md)
- [test_progress_percentage](../../../functions/src/plan/plan_tests/test_progress_percentage.md)
- [test_is_complete](../../../functions/src/plan/plan_tests/test_is_complete.md)
- [test_plan_state_transitions](../../../functions/src/plan/plan_tests/test_plan_state_transitions.md)
- [test_plan_rejection](../../../functions/src/plan/plan_tests/test_plan_rejection.md)
- [test_topological_sort_no_dependencies](../../../functions/src/plan/plan_tests/test_topological_sort_no_dependencies.md)
- [test_topological_sort_with_dependencies](../../../functions/src/plan/plan_tests/test_topological_sort_with_dependencies.md)
- [test_topological_sort_circular_dependency](../../../functions/src/plan/plan_tests/test_topological_sort_circular_dependency.md)
- [test_validate_dependencies_success](../../../functions/src/plan/plan_tests/test_validate_dependencies_success.md)
- [test_validate_dependencies_invalid_reference](../../../functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference.md)
- [test_validate_dependencies_circular](../../../functions/src/plan/plan_tests/test_validate_dependencies_circular.md)
- [test_task_state_transitions](../../../functions/src/plan/plan_tests/test_task_state_transitions.md)
- [test_task_failure](../../../functions/src/plan/plan_tests/test_task_failure.md)
- [test_task_blocking](../../../functions/src/plan/plan_tests/test_task_blocking.md)
- [test_task_skip](../../../functions/src/plan/plan_tests/test_task_skip.md)
- [test_task_complexity_stars](../../../functions/src/plan/plan_tests/test_task_complexity_stars.md)
- [test_task_type_display](../../../functions/src/plan/plan_tests/test_task_type_display.md)
- [test_task_status_display](../../../functions/src/plan/plan_tests/test_task_status_display.md)
- [test_task_status_icons](../../../functions/src/plan/plan_tests/test_task_status_icons.md)
- [test_plan_status_display](../../../functions/src/plan/plan_tests/test_plan_status_display.md)
- [test_complex_dependency_chain](../../../functions/src/plan/plan_tests/test_complex_dependency_chain.md)
- [auto_run_no_dialogs](../../../functions/src/plan/plan_tests/auto_run_no_dialogs.md)

# Imports

- `crate::plan::*`
- `uuid::Uuid`

# Member of

- [crustly](../../../packages/crustly.md)