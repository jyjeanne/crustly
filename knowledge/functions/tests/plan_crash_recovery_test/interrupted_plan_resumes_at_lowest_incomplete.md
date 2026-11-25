---
type: Rust Function
title: interrupted_plan_resumes_at_lowest_incomplete
resource: tests/plan_crash_recovery_test.rs#L224-L236
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/interrupted_plan_from_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_crash_recovery_test/minimal_task
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn interrupted_plan_resumes_at_lowest_incomplete()`

# Calls

- [interrupted_plan_from_tasks](../../../functions/src/db/models/interrupted_plan_from_tasks.md)
- [minimal_task](../../../functions/tests/plan_crash_recovery_test/minimal_task.md)