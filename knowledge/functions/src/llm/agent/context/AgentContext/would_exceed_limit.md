---
type: Rust Method
title: would_exceed_limit
resource: src/llm/agent/context.rs#L161-L163
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/context/AgentContext/trim_to_fit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/test_would_exceed_limit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn would_exceed_limit(&self, additional_tokens: usize) -> bool`

# Called by

- [trim_to_fit](../../../../../../functions/src/llm/agent/context/AgentContext/trim_to_fit.md)
- [test_would_exceed_limit](../../../../../../functions/src/llm/agent/context/test_would_exceed_limit.md)