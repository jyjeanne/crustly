---
type: Rust Method
title: completion_budget
resource: src/llm/agent/budget.rs#L145-L147
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/completion_budget_is_never_zero_and_never_above_the_cap
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

`pub fn completion_budget(&self) -> u32`

# Calls

- [available](../../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)

# Called by

- [completion_budget_is_never_zero_and_never_above_the_cap](../../../../../../functions/src/llm/agent/budget/completion_budget_is_never_zero_and_never_above_the_cap.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)