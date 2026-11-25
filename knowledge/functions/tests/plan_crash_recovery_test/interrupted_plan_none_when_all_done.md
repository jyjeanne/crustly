---
type: Rust Function
title: interrupted_plan_none_when_all_done
resource: tests/plan_crash_recovery_test.rs#L212-L220
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn interrupted_plan_none_when_all_done()`

# Calls

- [interrupted_plan_from_tasks](../../../functions/src/db/models/interrupted_plan_from_tasks.md)
- [minimal_task](../../../functions/tests/plan_crash_recovery_test/minimal_task.md)