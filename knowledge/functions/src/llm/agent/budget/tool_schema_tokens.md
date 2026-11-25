---
type: Rust Function
title: tool_schema_tokens
resource: src/llm/agent/budget.rs#L212-L219
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/budget/ContextBudget/measure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/RequestTools/new
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn tool_schema_tokens(tools: &[Tool]) -> u32`

# Calls

- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)

# Called by

- [measure](../../../../../functions/src/llm/agent/budget/ContextBudget/measure.md)
- [new](../../../../../functions/src/llm/agent/service/RequestTools/new.md)