---
type: Rust Function
title: test_end_to_end_plan_creation_and_retrieval
resource: tests/plan_mode_integration_test.rs#L127-L161
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
---

# Signature

`async fn test_end_to_end_plan_creation_and_retrieval()`

# Calls

- [setup_test_env](../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [create_multi_task_plan](../../../functions/tests/plan_mode_integration_test/create_multi_task_plan.md)