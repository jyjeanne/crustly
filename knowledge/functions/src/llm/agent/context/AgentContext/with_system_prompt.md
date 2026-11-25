---
type: Rust Method
title: with_system_prompt
resource: src/llm/agent/context.rs#L100-L104
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/estimate_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/context/test_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_system_prompt(mut self, prompt: String) -> Self`

# Calls

- [estimate_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_tokens.md)

# Called by

- [test_system_prompt](../../../../../../functions/src/llm/agent/context/test_system_prompt.md)