---
type: Rust Method
title: estimate_tokens
resource: src/llm/agent/context.rs#L146-L148
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/agent/context/token_count
  called_by:
  - functions/src/llm/agent/context/AgentContext/with_system_prompt
  - functions/src/llm/agent/context/AgentContext/estimate_message_tokens
  - functions/src/llm/agent/context/test_token_estimation
---

# Signature

`fn estimate_tokens(text: &str) -> usize`

# Calls

- [token_count](../../../../../../functions/src/llm/agent/context/token_count.md)

# Called by

- [with_system_prompt](../../../../../../functions/src/llm/agent/context/AgentContext/with_system_prompt.md)
- [estimate_message_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_message_tokens.md)
- [test_token_estimation](../../../../../../functions/src/llm/agent/context/test_token_estimation.md)