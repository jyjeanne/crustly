---
type: Rust Function
title: budget
resource: src/llm/agent/budget.rs#L239-L241
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/from_counts
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/calibration_scales_the_estimate_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/completion_budget_is_never_zero_and_never_above_the_cap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/min_answer_tokens_scales_with_the_window
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn budget(window: u32, system: u32, tools: u32, history: usize) -> ContextBudget`

# Calls

- [from_counts](../../../../../functions/src/llm/agent/budget/ContextBudget/from_counts.md)

# Called by

- [full_tool_set_leaves_little_of_an_8k_window](../../../../../functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window.md)
- [distinguishes_prompt_overflow_from_a_clamped_full_window](../../../../../functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window.md)
- [calibration_scales_the_estimate_up](../../../../../functions/src/llm/agent/budget/calibration_scales_the_estimate_up.md)
- [history_crowds_the_answer_only_when_dropping_it_would_help](../../../../../functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help.md)
- [completion_budget_is_never_zero_and_never_above_the_cap](../../../../../functions/src/llm/agent/budget/completion_budget_is_never_zero_and_never_above_the_cap.md)
- [min_answer_tokens_scales_with_the_window](../../../../../functions/src/llm/agent/budget/min_answer_tokens_scales_with_the_window.md)