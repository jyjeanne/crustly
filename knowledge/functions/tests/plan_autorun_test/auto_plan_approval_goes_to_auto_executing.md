---
type: Rust Function
title: auto_plan_approval_goes_to_auto_executing
resource: tests/plan_autorun_test.rs#L32-L65
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/plan/PlanModeState/approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_autorun_test/make_plan_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanModeState/tool_needs_approval
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn auto_plan_approval_goes_to_auto_executing()`

# Calls

- [approve](../../../functions/src/plan/PlanModeState/approve.md)
- [make_plan_task](../../../functions/tests/plan_autorun_test/make_plan_task.md)
- [tool_needs_approval](../../../functions/src/plan/PlanModeState/tool_needs_approval.md)