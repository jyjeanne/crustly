---
type: Rust Method
title: estimate_tokens
resource: src/llm/agent/context.rs#L196-L198
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/context/AgentContext/with_system_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/estimate_message_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/test_token_estimation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn estimate_tokens(text: &str) -> usize`

# Calls

- [token_count](../../../../../../functions/src/llm/agent/context/token_count.md)

# Called by

- [with_system_prompt](../../../../../../functions/src/llm/agent/context/AgentContext/with_system_prompt.md)
- [estimate_message_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_message_tokens.md)
- [test_token_estimation](../../../../../../functions/src/llm/agent/context/test_token_estimation.md)