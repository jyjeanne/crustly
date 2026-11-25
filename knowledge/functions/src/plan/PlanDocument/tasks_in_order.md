---
type: Rust Method
title: tasks_in_order
resource: src/plan/mod.rs#L86-L145
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/plan/PlanDocument/validate_dependencies
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
  - target: functions/src/plan/plan_tests/test_complex_dependency_chain
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/execute_next_plan_task
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn tasks_in_order(&self) -> Option<Vec<&PlanTask>>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [validate_dependencies](../../../../functions/src/plan/PlanDocument/validate_dependencies.md)
- [test_topological_sort_no_dependencies](../../../../functions/src/plan/plan_tests/test_topological_sort_no_dependencies.md)
- [test_topological_sort_with_dependencies](../../../../functions/src/plan/plan_tests/test_topological_sort_with_dependencies.md)
- [test_topological_sort_circular_dependency](../../../../functions/src/plan/plan_tests/test_topological_sort_circular_dependency.md)
- [test_complex_dependency_chain](../../../../functions/src/plan/plan_tests/test_complex_dependency_chain.md)
- [execute_next_plan_task](../../../../functions/src/tui/app/App/execute_next_plan_task.md)