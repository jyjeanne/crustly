---
type: Rust Function
title: test_task_blocking_and_failure_scenarios
resource: tests/plan_mode_integration_test.rs#L397-L431
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/tests/plan_mode_integration_test/setup_test_env
  - functions/tests/plan_mode_integration_test/create_multi_task_plan
  - functions/src/plan/PlanDocument/get_task_mut
---

# Signature

`async fn test_task_blocking_and_failure_scenarios()`

# Calls

- [setup_test_env](../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [create_multi_task_plan](../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)
- [get_task_mut](../../../functions/src/plan/PlanDocument/get_task_mut.md)