---
type: Rust Method
title: estimated_prompt_tokens
resource: src/llm/agent/budget.rs#L102-L105
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/calibrated
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/raw_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/ContextBudget/used
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/overflow_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/finish
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn estimated_prompt_tokens(&self) -> u32`

# Calls

- [calibrated](../../../../../../functions/src/llm/agent/budget/ContextBudget/calibrated.md)
- [raw_prompt_tokens](../../../../../../functions/src/llm/agent/budget/ContextBudget/raw_prompt_tokens.md)

# Called by

- [used](../../../../../../functions/src/llm/agent/budget/ContextBudget/used.md)
- [overflow_error](../../../../../../functions/src/llm/agent/budget/ContextBudget/overflow_error.md)
- [distinguishes_prompt_overflow_from_a_clamped_full_window](../../../../../../functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window.md)
- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)