---
type: Rust Method
title: validate_dependencies
resource: src/plan/mod.rs#L208-L247
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/PlanDocument/tasks_in_order
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/plan_tool/PlanTool/tool/execute
  - functions/src/plan/plan_tests/test_validate_dependencies_success
  - functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference
  - functions/src/plan/plan_tests/test_validate_dependencies_circular
  - functions/src/plan/plan_tests/test_complex_dependency_chain
---

# Signature

`pub fn validate_dependencies(&self) -> Result<(), String>`

# Calls

- [tasks_in_order](../../../../functions/src/plan/PlanDocument/tasks_in_order.md)
- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [test_validate_dependencies_success](../../../../functions/src/plan/plan_tests/test_validate_dependencies_success.md)
- [test_validate_dependencies_invalid_reference](../../../../functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference.md)
- [test_validate_dependencies_circular](../../../../functions/src/plan/plan_tests/test_validate_dependencies_circular.md)
- [test_complex_dependency_chain](../../../../functions/src/plan/plan_tests/test_complex_dependency_chain.md)