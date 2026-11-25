---
type: Rust Method
title: is_high_risk_tool
resource: src/plan/mod.rs#L943-L948
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/auto_mode_bypasses_approval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanModeState/tool_needs_approval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_autorun_test/high_risk_tools_pause_auto_execution
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_high_risk_tool(tool_name: &str) -> bool`

# Called by

- [auto_mode_bypasses_approval](../../../../functions/src/cli/auto_mode_bypasses_approval.md)
- [tool_needs_approval](../../../../functions/src/plan/PlanModeState/tool_needs_approval.md)
- [high_risk_tools_pause_auto_execution](../../../../functions/tests/plan_autorun_test/high_risk_tools_pause_auto_execution.md)