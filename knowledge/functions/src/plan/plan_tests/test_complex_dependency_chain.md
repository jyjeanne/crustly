---
type: Rust Function
title: test_complex_dependency_chain
resource: src/plan/plan_tests.rs#L478-L537
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/plan_tests/create_test_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/validate_dependencies
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/tasks_in_order
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_complex_dependency_chain()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [validate_dependencies](../../../../functions/src/plan/PlanDocument/validate_dependencies.md)
- [tasks_in_order](../../../../functions/src/plan/PlanDocument/tasks_in_order.md)