---
type: Rust Method
title: overflow_error
resource: src/llm/agent/budget.rs#L136-L141
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
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

`pub fn overflow_error(&self) -> AgentError`

# Calls

- [estimated_prompt_tokens](../../../../../../functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens.md)

# Called by

- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)