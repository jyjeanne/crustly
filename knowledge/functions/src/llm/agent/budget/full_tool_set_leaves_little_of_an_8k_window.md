---
type: Rust Function
title: full_tool_set_leaves_little_of_an_8k_window
resource: src/llm/agent/budget.rs#L247-L253
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
  - target: functions/src/llm/agent/budget/ContextBudget/fixed_overhead
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextUsage/overhead_is_tight
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn full_tool_set_leaves_little_of_an_8k_window()`

# Calls

- [budget](../../../../../functions/src/llm/agent/budget/budget.md)
- [fixed_overhead](../../../../../functions/src/llm/agent/budget/ContextBudget/fixed_overhead.md)
- [available](../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)
- [is_tight](../../../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)
- [usage](../../../../../functions/src/llm/agent/budget/ContextBudget/usage.md)
- [overhead_is_tight](../../../../../functions/src/llm/agent/budget/ContextUsage/overhead_is_tight.md)