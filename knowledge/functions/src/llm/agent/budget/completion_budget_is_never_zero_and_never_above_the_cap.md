---
type: Rust Function
title: completion_budget_is_never_zero_and_never_above_the_cap
resource: src/llm/agent/budget.rs#L256-L262
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
  - target: functions/src/llm/agent/budget/ContextBudget/completion_budget
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn completion_budget_is_never_zero_and_never_above_the_cap()`

# Calls

- [budget](../../../../../functions/src/llm/agent/budget/budget.md)
- [completion_budget](../../../../../functions/src/llm/agent/budget/ContextBudget/completion_budget.md)