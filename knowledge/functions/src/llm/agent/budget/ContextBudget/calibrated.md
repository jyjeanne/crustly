---
type: Rust Method
title: calibrated
resource: src/llm/agent/budget.rs#L95-L97
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/fixed_overhead
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn calibrated(&self, tokens: u32) -> u32`

# Called by

- [estimated_prompt_tokens](../../../../../../functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens.md)
- [fixed_overhead](../../../../../../functions/src/llm/agent/budget/ContextBudget/fixed_overhead.md)