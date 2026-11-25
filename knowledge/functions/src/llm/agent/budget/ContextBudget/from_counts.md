---
type: Rust Method
title: from_counts
resource: src/llm/agent/budget.rs#L70-L86
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/budget/ContextBudget/measure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/calibration_scales_the_estimate_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/calibration_never_goes_below_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/measure_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn from_counts( window: u32, system_prompt_tokens: u32, tool_schema_tokens: u32, tool_count: u32, history_tokens: usize, calibration: f64, ) -> Self`

# Called by

- [measure](../../../../../../functions/src/llm/agent/budget/ContextBudget/measure.md)
- [budget](../../../../../../functions/src/llm/agent/budget/budget.md)
- [calibration_scales_the_estimate_up](../../../../../../functions/src/llm/agent/budget/calibration_scales_the_estimate_up.md)
- [calibration_never_goes_below_one](../../../../../../functions/src/llm/agent/budget/calibration_never_goes_below_one.md)
- [measure_budget](../../../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [calibration_learns_from_the_provider_and_resets_on_switch](../../../../../../functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch.md)