---
type: Rust Function
title: test_validate_dependencies_invalid_reference
resource: src/plan/plan_tests.rs#L297-L308
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
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_validate_dependencies_invalid_reference()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [validate_dependencies](../../../../functions/src/plan/PlanDocument/validate_dependencies.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)