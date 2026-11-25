---
type: Rust Method
title: used
resource: src/llm/agent/budget.rs#L118-L120
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/calibration_scales_the_estimate_up
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn used(&self) -> u32`

# Calls

- [estimated_prompt_tokens](../../../../../../functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens.md)

# Called by

- [available](../../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)
- [usage](../../../../../../functions/src/llm/agent/budget/ContextBudget/usage.md)
- [calibration_scales_the_estimate_up](../../../../../../functions/src/llm/agent/budget/calibration_scales_the_estimate_up.md)