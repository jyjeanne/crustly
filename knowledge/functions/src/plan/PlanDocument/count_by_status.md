---
type: Rust Method
title: count_by_status
resource: src/plan/mod.rs#L159-L161
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/plan/PlanDocument/progress_percentage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_count_by_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn count_by_status(&self, status: TaskStatus) -> usize`

# Called by

- [progress_percentage](../../../../functions/src/plan/PlanDocument/progress_percentage.md)
- [test_count_by_status](../../../../functions/src/plan/plan_tests/test_count_by_status.md)