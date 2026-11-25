---
type: Rust Method
title: history_crowds_answer
resource: src/llm/agent/budget.rs#L165-L168
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/fixed_overhead
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/min_answer_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn history_crowds_answer(&self) -> bool`

# Calls

- [is_tight](../../../../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)
- [fixed_overhead](../../../../../../functions/src/llm/agent/budget/ContextBudget/fixed_overhead.md)
- [min_answer_tokens](../../../../../../functions/src/llm/agent/budget/ContextBudget/min_answer_tokens.md)

# Called by

- [history_crowds_the_answer_only_when_dropping_it_would_help](../../../../../../functions/src/llm/agent/budget/history_crowds_the_answer_only_when_dropping_it_would_help.md)
- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)