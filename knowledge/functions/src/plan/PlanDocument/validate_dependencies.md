---
type: Rust Method
title: validate_dependencies
resource: src/plan/mod.rs#L208-L247
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanDocument/tasks_in_order
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
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
  - target: functions/src/plan/plan_tests/test_complex_dependency_chain
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_dependencies(&self) -> Result<(), String>`

# Calls

- [tasks_in_order](../../../../functions/src/plan/PlanDocument/tasks_in_order.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [test_validate_dependencies_success](../../../../functions/src/plan/plan_tests/test_validate_dependencies_success.md)
- [test_validate_dependencies_invalid_reference](../../../../functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference.md)
- [test_validate_dependencies_circular](../../../../functions/src/plan/plan_tests/test_validate_dependencies_circular.md)
- [test_complex_dependency_chain](../../../../functions/src/plan/plan_tests/test_complex_dependency_chain.md)