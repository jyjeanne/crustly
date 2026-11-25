---
type: Rust Method
title: call_model
resource: src/llm/agent/service/run.rs#L355-L430
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/measure_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_full
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/history_crowds_answer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/compact_deeper
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/fall_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/overflow_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/build_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/completion_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/call_provider_streaming
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/error/ProviderError/is_context_overflow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn call_model(&mut self, totals: &mut Totals) -> Result<(LLMResponse, ContextBudget)>`

# Calls

- [measure_budget](../../../../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [is_full](../../../../../../../functions/src/llm/agent/budget/ContextBudget/is_full.md)
- [history_crowds_answer](../../../../../../../functions/src/llm/agent/budget/ContextBudget/history_crowds_answer.md)
- [compact_deeper](../../../../../../../functions/src/llm/agent/service/run/AgentRun/compact_deeper.md)
- [estimated_prompt_tokens](../../../../../../../functions/src/llm/agent/budget/ContextBudget/estimated_prompt_tokens.md)
- [fall_back](../../../../../../../functions/src/llm/agent/service/run/AgentRun/fall_back.md)
- [overflow_error](../../../../../../../functions/src/llm/agent/budget/ContextBudget/overflow_error.md)
- [build_request](../../../../../../../functions/src/llm/agent/service/AgentService/build_request.md)
- [completion_budget](../../../../../../../functions/src/llm/agent/budget/ContextBudget/completion_budget.md)
- [call_provider_streaming](../../../../../../../functions/src/llm/agent/service/AgentService/call_provider_streaming.md)
- [is_context_overflow](../../../../../../../functions/src/llm/provider/error/ProviderError/is_context_overflow.md)
- [available](../../../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)

# Called by

- [run](../../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)