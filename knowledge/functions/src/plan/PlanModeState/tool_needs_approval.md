---
type: Rust Method
title: tool_needs_approval
resource: src/plan/mod.rs#L864-L874
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/PlanModeState/is_high_risk_tool
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/plan_autorun_test/auto_plan_approval_goes_to_auto_executing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_autorun_test/interactive_approval_goes_to_executing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn tool_needs_approval(&self, tool_name: &str, _threshold: u8) -> bool`

# Calls

- [is_high_risk_tool](../../../../functions/src/plan/PlanModeState/is_high_risk_tool.md)

# Called by

- [auto_plan_approval_goes_to_auto_executing](../../../../functions/tests/plan_autorun_test/auto_plan_approval_goes_to_auto_executing.md)
- [interactive_approval_goes_to_executing](../../../../functions/tests/plan_autorun_test/interactive_approval_goes_to_executing.md)