---
type: Rust Method
title: raw_prompt_tokens
resource: src/llm/agent/budget.rs#L89-L93
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/record_calibration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn raw_prompt_tokens(&self) -> u32`

# Called by

- [estimated_prompt_tokens](../../../../../../functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens.md)
- [record_calibration](../../../../../../functions/src/llm/agent/service/AgentService/record_calibration.md)