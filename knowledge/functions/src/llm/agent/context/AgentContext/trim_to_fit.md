---
type: Rust Method
title: trim_to_fit
resource: src/llm/agent/context.rs#L165-L174
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/context/AgentContext/would_exceed_limit
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/agent/context/AgentContext/estimate_message_tokens
  called_by:
  - functions/src/llm/agent/context/test_trim_to_fit
---

# Signature

`pub fn trim_to_fit(&mut self, required_space: usize)`

# Calls

- [would_exceed_limit](../../../../../../functions/src/llm/agent/context/AgentContext/would_exceed_limit.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [estimate_message_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_message_tokens.md)

# Called by

- [test_trim_to_fit](../../../../../../functions/src/llm/agent/context/test_trim_to_fit.md)