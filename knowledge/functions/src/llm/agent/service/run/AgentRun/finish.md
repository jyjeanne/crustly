---
type: Rust Method
title: finish
resource: src/llm/agent/service/run.rs#L507-L578
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/RequestTools/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/measure_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_full
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/overflow_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/completion_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/build_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/add_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn finish( &mut self, reason: &str, pending: Option<LLMResponse>, totals: &mut Totals, ) -> Result<LLMResponse>`

# Calls

- [add_message](../../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user](../../../../../../../functions/src/llm/provider/types/Message/user.md)
- [new](../../../../../../../functions/src/llm/agent/service/RequestTools/new.md)
- [make_room](../../../../../../../functions/src/llm/agent/service/AgentService/make_room.md)
- [measure_budget](../../../../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [is_full](../../../../../../../functions/src/llm/agent/budget/ContextBudget/is_full.md)
- [overflow_error](../../../../../../../functions/src/llm/agent/budget/ContextBudget/overflow_error.md)
- [completion_budget](../../../../../../../functions/src/llm/agent/budget/ContextBudget/completion_budget.md)
- [build_request](../../../../../../../functions/src/llm/agent/service/AgentService/build_request.md)
- [add_usage](../../../../../../../functions/src/llm/agent/service/run/AgentRun/add_usage.md)
- [estimated_prompt_tokens](../../../../../../../functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens.md)

# Called by

- [run](../../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)