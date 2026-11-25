---
type: Rust Function
title: test_get_task
resource: src/plan/plan_tests.rs#L61-L74
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
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_get_task()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [get_task](../../../../functions/src/plan/PlanDocument/get_task.md)