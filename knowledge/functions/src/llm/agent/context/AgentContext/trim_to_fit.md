---
type: Rust Method
title: trim_to_fit
resource: src/llm/agent/context.rs#L216-L225
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/would_exceed_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/estimate_message_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/context/test_trim_to_fit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn trim_to_fit(&mut self, required_space: usize)`

# Calls

- [would_exceed_limit](../../../../../../functions/src/llm/agent/context/AgentContext/would_exceed_limit.md)
- [estimate_message_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_message_tokens.md)

# Called by

- [test_trim_to_fit](../../../../../../functions/src/llm/agent/context/test_trim_to_fit.md)