---
type: Rust Method
title: is_full
resource: src/llm/agent/budget.rs#L131-L133
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
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

`pub fn is_full(&self) -> bool`

# Calls

- [available](../../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)

# Called by

- [distinguishes_prompt_overflow_from_a_clamped_full_window](../../../../../../functions/src/llm/agent/budget/distinguishes_prompt_overflow_from_a_clamped_full_window.md)
- [history_crowds_the_answer_only_when_dropping_it_would_help](../../../../../../functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)