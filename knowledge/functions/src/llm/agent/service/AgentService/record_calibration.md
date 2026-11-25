---
type: Rust Method
title: record_calibration
resource: src/llm/agent/service.rs#L905-L917
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/calibration_from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/raw_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn record_calibration(&self, response: &LLMResponse, budget: &ContextBudget)`

# Calls

- [calibration_from](../../../../../../functions/src/llm/agent/budget/calibration_from.md)
- [raw_prompt_tokens](../../../../../../functions/src/llm/agent/budget/ContextBudget/raw_prompt_tokens.md)

# Called by

- [send_message](../../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [calibration_learns_from_the_provider_and_resets_on_switch](../../../../../../functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch.md)
- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)