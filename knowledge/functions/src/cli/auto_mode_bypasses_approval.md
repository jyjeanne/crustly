---
type: Rust Function
title: auto_mode_bypasses_approval
resource: src/cli/mod.rs#L896-L905
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/PlanModeState/is_high_risk_tool
  called_by:
  - functions/src/cli/build_approval_callback
---

# Signature

`fn auto_mode_bypasses_approval(mode: &crate::config::PlanExecMode, tool_name: &str) -> bool`

# Calls

- [is_high_risk_tool](../../../functions/src/plan/PlanModeState/is_high_risk_tool.md)

# Called by

- [build_approval_callback](../../../functions/src/cli/build_approval_callback.md)