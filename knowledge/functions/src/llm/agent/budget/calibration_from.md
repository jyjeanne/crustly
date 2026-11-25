---
type: Rust Function
title: calibration_from
resource: src/llm/agent/budget.rs#L227-L233
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/budget/calibration_from_keeps_upward_corrections_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/record_calibration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn calibration_from(provider_prompt_tokens: u32, estimated_prompt_tokens: u32) -> Option<f64>`

# Called by

- [calibration_from_keeps_upward_corrections_only](../../../../../functions/src/llm/agent/budget/calibration_from_keeps_upward_corrections_only.md)
- [record_calibration](../../../../../functions/src/llm/agent/service/AgentService/record_calibration.md)