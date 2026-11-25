---
type: Rust Function
title: test_count_by_status
resource: src/plan/plan_tests.rs#L94-L113
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/plan_tests/create_test_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/add_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/count_by_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_count_by_status()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [count_by_status](../../../../functions/src/plan/PlanDocument/count_by_status.md)