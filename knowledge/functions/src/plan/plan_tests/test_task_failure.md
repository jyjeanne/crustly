---
type: Rust Function
title: test_task_failure
resource: src/plan/plan_tests.rs#L349-L356
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/plan_tests/create_test_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/fail
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_task_failure()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [fail](../../../../functions/src/plan/PlanTask/fail.md)