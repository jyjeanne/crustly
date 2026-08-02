---
type: Rust Function
title: test_plan_state_transition_workflow
resource: tests/plan_mode_integration_test.rs#L164-L218
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/tests/plan_mode_integration_test/setup_test_env
  - functions/tests/plan_mode_integration_test/create_multi_task_plan
  - functions/src/plan/PlanDocument/get_task_mut
---

# Signature

`async fn test_plan_state_transition_workflow()`

# Calls

- [setup_test_env](../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [create_multi_task_plan](../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)
- [get_task_mut](../../../functions/src/plan/PlanDocument/get_task_mut.md)