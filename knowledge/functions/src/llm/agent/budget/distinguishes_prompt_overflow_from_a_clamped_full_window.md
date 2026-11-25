---
type: Rust Function
title: distinguishes_prompt_overflow_from_a_clamped_full_window
resource: src/llm/agent/budget.rs#L265-L270
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_full
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn distinguishes_prompt_overflow_from_a_clamped_full_window()`

# Calls

- [budget](../../../../../functions/src/llm/agent/budget/budget.md)
- [available](../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)
- [is_full](../../../../../functions/src/llm/agent/budget/ContextBudget/is_full.md)
- [estimated_prompt_tokens](../../../../../functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens.md)