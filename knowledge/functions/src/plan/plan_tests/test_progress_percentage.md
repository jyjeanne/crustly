---
type: Rust Function
title: test_progress_percentage
resource: src/plan/plan_tests.rs#L116-L142
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
  - target: functions/src/plan/PlanDocument/progress_percentage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_progress_percentage()`

# Calls

- [create_test_task](../../../../functions/src/plan/plan_tests/create_test_task.md)
- [add_task](../../../../functions/src/plan/PlanDocument/add_task.md)
- [progress_percentage](../../../../functions/src/plan/PlanDocument/progress_percentage.md)