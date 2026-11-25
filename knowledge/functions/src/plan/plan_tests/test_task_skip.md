---
type: Rust Function
title: test_task_skip
resource: src/plan/plan_tests.rs#L373-L379
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
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_task_skip()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [skip](../../../../functions/src/plan/PlanTask/skip.md)