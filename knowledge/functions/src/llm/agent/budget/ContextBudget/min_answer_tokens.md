---
type: Rust Method
title: min_answer_tokens
resource: src/llm/agent/budget.rs#L151-L153
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/budget/ContextBudget/is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/history_crowds_answer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/min_answer_tokens_scales_with_the_window
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn min_answer_tokens(&self) -> u32`

# Called by

- [is_tight](../../../../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)
- [history_crowds_answer](../../../../../../functions/src/llm/agent/budget/ContextBudget/history_crowds_answer.md)
- [min_answer_tokens_scales_with_the_window](../../../../../../functions/src/llm/agent/budget/min_answer_tokens_scales_with_the_window.md)