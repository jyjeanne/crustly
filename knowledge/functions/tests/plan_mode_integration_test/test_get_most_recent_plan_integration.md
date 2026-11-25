---
type: Rust Function
title: test_get_most_recent_plan_integration
resource: tests/plan_mode_integration_test.rs#L434-L467
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/get_most_recent_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/create_multi_task_plan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_get_most_recent_plan_integration()`

# Calls

- [setup_test_env](../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [get_most_recent_plan](../../../functions/src/services/plan/PlanService/get_most_recent_plan.md)
- [create_multi_task_plan](../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)