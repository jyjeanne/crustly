---
type: Rust Function
title: calibration_scales_the_estimate_up
resource: src/llm/agent/budget.rs#L273-L278
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
  - target: functions/src/llm/agent/budget/ContextBudget/from_counts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/used
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn calibration_scales_the_estimate_up()`

# Calls

- [budget](../../../../../functions/src/llm/agent/budget/budget.md)
- [from_counts](../../../../../functions/src/llm/agent/budget/ContextBudget/from_counts.md)
- [used](../../../../../functions/src/llm/agent/budget/ContextBudget/used.md)
- [available](../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)