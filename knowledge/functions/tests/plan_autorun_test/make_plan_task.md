---
type: Rust Function
title: make_plan_task
resource: tests/plan_autorun_test.rs#L8-L27
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/plan_autorun_test/auto_plan_approval_goes_to_auto_executing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_autorun_test/interactive_approval_goes_to_executing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_plan_task(order: usize, title: &str) -> PlanTask`

# Called by

- [auto_plan_approval_goes_to_auto_executing](../../../functions/tests/plan_autorun_test/auto_plan_approval_goes_to_auto_executing.md)
- [interactive_approval_goes_to_executing](../../../functions/tests/plan_autorun_test/interactive_approval_goes_to_executing.md)