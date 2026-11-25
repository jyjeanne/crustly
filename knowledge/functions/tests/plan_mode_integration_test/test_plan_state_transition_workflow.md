---
type: Rust Function
title: test_plan_state_transition_workflow
resource: tests/plan_mode_integration_test.rs#L164-L218
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/create_multi_task_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task_mut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/is_complete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_plan_state_transition_workflow()`

# Calls

- [setup_test_env](../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [create_multi_task_plan](../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)
- [get_task_mut](../../../functions/src/plan/PlanDocument/get_task_mut.md)
- [is_complete](../../../functions/src/plan/PlanDocument/is_complete.md)