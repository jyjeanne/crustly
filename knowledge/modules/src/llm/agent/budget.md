---
type: Rust Module
title: budget
resource: src/llm/agent/budget.rs#L1-L319
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-context-token-count
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-error-agenterror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-tool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ContextBudget](../../../../classes/src/llm/agent/budget/ContextBudget.md)
- [measure](../../../../functions/src/llm/agent/budget/ContextBudget/measure.md)
- [from_counts](../../../../functions/src/llm/agent/budget/ContextBudget/from_counts.md)
- [raw_prompt_tokens](../../../../functions/src/llm/agent/budget/ContextBudget/raw_prompt_tokens.md)
- [calibrated](../../../../functions/src/llm/agent/budget/ContextBudget/calibrated.md)
- [estimated_prompt_tokens](../../../../functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens.md)
- [fixed_overhead](../../../../functions/src/llm/agent/budget/ContextBudget/fixed_overhead.md)
- [used](../../../../functions/src/llm/agent/budget/ContextBudget/used.md)
- [available](../../../../functions/src/llm/agent/budget/ContextBudget/available.md)
- [is_full](../../../../functions/src/llm/agent/budget/ContextBudget/is_full.md)
- [overflow_error](../../../../functions/src/llm/agent/budget/ContextBudget/overflow_error.md)
- [completion_budget](../../../../functions/src/llm/agent/budget/ContextBudget/completion_budget.md)
- [min_answer_tokens](../../../../functions/src/llm/agent/budget/ContextBudget/min_answer_tokens.md)
- [is_tight](../../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)
- [history_crowds_answer](../../../../functions/src/llm/agent/budget/ContextBudget/history_crowds_answer.md)
- [usage](../../../../functions/src/llm/agent/budget/ContextBudget/usage.md)
- [ContextUsage](../../../../classes/src/llm/agent/budget/ContextUsage.md)
- [overhead_is_tight](../../../../functions/src/llm/agent/budget/ContextUsage/overhead_is_tight.md)
- [tool_schema_tokens](../../../../functions/src/llm/agent/budget/tool_schema_tokens.md)
- [calibration_from](../../../../functions/src/llm/agent/budget/calibration_from.md)
- [budget](../../../../functions/src/llm/agent/budget/budget.md)
- [full_tool_set_leaves_little_of_an_8k_window](../../../../functions/src/llm/agent/budget/full_tool_set_leaves_little_of_an_8k_window.md)
- [completion_budget_is_never_zero_and_never_above_the_cap](../../../../functions/src/llm/agent/budget/completion_budget_is_never_zero_and_never_above_the_cap.md)
- [distinguishes_prompt_overflow_from_a_clamped_full_window](../../../../functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window.md)
- [calibration_scales_the_estimate_up](../../../../functions/src/llm/agent/budget/calibration_scales_the_estimate_up.md)
- [calibration_never_goes_below_one](../../../../functions/src/llm/agent/budget/calibration_never_goes_below_one.md)
- [calibration_from_keeps_upward_corrections_only](../../../../functions/src/llm/agent/budget/calibration_from_keeps_upward_corrections_only.md)
- [history_crowds_the_answer_only_when_dropping_it_would_help](../../../../functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help.md)
- [min_answer_tokens_scales_with_the_window](../../../../functions/src/llm/agent/budget/min_answer_tokens_scales_with_the_window.md)

# Imports

- `super::context::token_count`
- `super::error::AgentError`
- `crate::llm::provider::Tool`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)