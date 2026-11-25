---
type: Rust Method
title: approve
resource: src/plan/mod.rs#L879-L895
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/plan/plan_tests/auto_run_no_dialogs
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_autorun_test/auto_plan_approval_goes_to_auto_executing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_autorun_test/interactive_approval_goes_to_executing
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn approve(plan_id: uuid::Uuid, tasks: Vec<PlanTask>, auto_plan: bool) -> Self`

# Called by

- [auto_run_no_dialogs](../../../../functions/src/plan/plan_tests/auto_run_no_dialogs.md)
- [auto_plan_approval_goes_to_auto_executing](../../../../functions/tests/plan_autorun_test/auto_plan_approval_goes_to_auto_executing.md)
- [interactive_approval_goes_to_executing](../../../../functions/tests/plan_autorun_test/interactive_approval_goes_to_executing.md)