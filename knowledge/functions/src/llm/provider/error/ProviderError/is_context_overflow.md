---
type: Rust Method
title: is_context_overflow
resource: src/llm/provider/error.rs#L82-L96
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/error/mentions_context_overflow
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/error/recognises_context_overflow_from_each_provider
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_context_overflow(&self) -> bool`

# Calls

- [mentions_context_overflow](../../../../../../functions/src/llm/provider/error/mentions_context_overflow.md)

# Called by

- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [recognises_context_overflow_from_each_provider](../../../../../../functions/src/llm/provider/error/recognises_context_overflow_from_each_provider.md)