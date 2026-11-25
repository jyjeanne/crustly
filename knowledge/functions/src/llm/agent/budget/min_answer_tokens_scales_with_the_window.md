---
type: Rust Function
title: min_answer_tokens_scales_with_the_window
resource: src/llm/agent/budget.rs#L314-L318
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/min_answer_tokens
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn min_answer_tokens_scales_with_the_window()`

# Calls

- [budget](../../../../../functions/src/llm/agent/budget/budget.md)
- [min_answer_tokens](../../../../../functions/src/llm/agent/budget/ContextBudget/min_answer_tokens.md)