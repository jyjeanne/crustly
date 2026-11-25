---
type: Rust Method
title: calibration
resource: src/llm/agent/service.rs#L895-L900
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
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

`fn calibration(&self) -> f64`

# Called by

- [context_usage_preview](../../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [measure_budget](../../../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [calibration_learns_from_the_provider_and_resets_on_switch](../../../../../../functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch.md)