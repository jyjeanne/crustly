---
type: Rust Method
title: is_high_risk_tool
resource: src/plan/mod.rs#L937-L942
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/cli/auto_mode_bypasses_approval
  - functions/src/plan/PlanModeState/tool_needs_approval
---

# Signature

`pub fn is_high_risk_tool(tool_name: &str) -> bool`

# Called by

- [auto_mode_bypasses_approval](../../../../functions/src/cli/auto_mode_bypasses_approval.md)
- [tool_needs_approval](../../../../functions/src/plan/PlanModeState/tool_needs_approval.md)