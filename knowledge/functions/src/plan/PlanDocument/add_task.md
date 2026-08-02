---
type: Rust Method
title: add_task
resource: src/plan/mod.rs#L79-L82
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/db/repository/plan/create_test_plan
  - functions/src/db/repository/plan/test_plan_update
  - functions/src/db/repository/plan/test_plan_with_complex_task_graph
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
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
  - functions/src/plan/plan_tests/test_complex_dependency_chain
  - functions/src/services/plan/create_test_plan
  - functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution
  - functions/tests/plan_mode_integration_test/create_multi_task_plan
---

# Signature

`pub fn add_task(&mut self, task: PlanTask)`

# Called by

- [create_test_plan](../../../../functions/src/db/repository/plan/create_test_plan.md)
- [test_plan_update](../../../../functions/src/db/repository/plan/test_plan_update.md)
- [test_plan_with_complex_task_graph](../../../../functions/src/db/repository/plan/test_plan_with_complex_task_graph.md)
- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
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
- [test_complex_dependency_chain](../../../../functions/src/plan/plan_tests/test_complex_dependency_chain.md)
- [create_test_plan](../../../../functions/src/services/plan/create_test_plan.md)
- [plan_task_error_marks_task_failed_and_stops_auto_execution](../../../../functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution.md)
- [create_multi_task_plan](../../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)