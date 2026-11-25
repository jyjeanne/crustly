---
type: Rust Method
title: is_complete
resource: src/plan/mod.rs#L173-L179
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_complete(&self) -> bool`

# Calls

- [all](../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [test_is_complete](../../../../functions/src/plan/plan_tests/test_is_complete.md)
- [test_plan_state_transition_workflow](../../../../functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow.md)