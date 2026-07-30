---
type: Rust Method
title: get_task_mut
resource: src/plan/mod.rs#L153-L156
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/db/repository/plan/test_plan_update_task_status
  - functions/src/plan/plan_tests/test_get_task_mut
  - functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow
  - functions/tests/plan_mode_integration_test/test_task_blocking_and_failure_scenarios
---

# Signature

`pub fn get_task_mut(&mut self, task_id: &Uuid) -> Option<&mut PlanTask>`

# Called by

- [test_plan_update_task_status](../../../../functions/src/db/repository/plan/test_plan_update_task_status.md)
- [test_get_task_mut](../../../../functions/src/plan/plan_tests/test_get_task_mut.md)
- [test_plan_state_transition_workflow](../../../../functions/tests/plan_mode_integration_test/test_plan_state_transition_workflow.md)
- [test_task_blocking_and_failure_scenarios](../../../../functions/tests/plan_mode_integration_test/test_task_blocking_and_failure_scenarios.md)