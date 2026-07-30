---
type: Rust Method
title: tool_needs_approval
resource: src/plan/mod.rs#L864-L874
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/plan/PlanModeState/is_high_risk_tool
---

# Signature

`pub fn tool_needs_approval(&self, tool_name: &str, _threshold: u8) -> bool`

# Calls

- [is_high_risk_tool](../../../../functions/src/plan/PlanModeState/is_high_risk_tool.md)