---
type: Rust Function
title: test_plan_rejection_workflow
resource: tests/plan_mode_integration_test.rs#L377-L394
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
  - target: functions/tests/plan_mode_integration_test/create_multi_task_plan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_plan_rejection_workflow()`

# Calls

- [setup_test_env](../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [create_multi_task_plan](../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)