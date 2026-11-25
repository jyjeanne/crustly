---
type: Rust Function
title: history_crowds_the_answer_only_when_dropping_it_would_help
resource: src/llm/agent/budget.rs#L296-L311
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
  - target: functions/src/llm/agent/budget/ContextBudget/is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_full
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/history_crowds_answer
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn history_crowds_the_answer_only_when_dropping_it_would_help()`

# Calls

- [budget](../../../../../functions/src/llm/agent/budget/budget.md)
- [is_tight](../../../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)
- [is_full](../../../../../functions/src/llm/agent/budget/ContextBudget/is_full.md)
- [history_crowds_answer](../../../../../functions/src/llm/agent/budget/ContextBudget/history_crowds_answer.md)