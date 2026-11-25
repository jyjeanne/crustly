---
type: Rust Function
title: test_task_blocking
resource: src/plan/plan_tests.rs#L359-L370
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
  - target: functions/src/plan/PlanTask/block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_task_blocking()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)