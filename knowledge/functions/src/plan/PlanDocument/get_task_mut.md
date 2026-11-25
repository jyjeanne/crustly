---
type: Rust Method
title: get_task_mut
resource: src/plan/mod.rs#L153-L156
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/plan/test_plan_update_task_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_get_task_mut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_task_blocking_and_failure_scenarios
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_task_mut(&mut self, task_id: &Uuid) -> Option<&mut PlanTask>`

# Calls

- [find](../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [test_plan_update_task_status](../../../../functions/src/db/repository/plan/test_plan_update_task_status.md)
- [test_get_task_mut](../../../../functions/src/plan/plan_tests/test_get_task_mut.md)
- [test_plan_state_transition_workflow](../../../../functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow.md)
- [test_task_blocking_and_failure_scenarios](../../../../functions/tests/plan_mode_integration_test/test_task_blocking_and_failure_scenarios.md)