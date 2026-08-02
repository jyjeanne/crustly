---
type: Rust Method
title: estimate_message_tokens
resource: src/llm/agent/context.rs#L116-L143
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/context/AgentContext/estimate_tokens
  called_by:
  - functions/src/llm/agent/context/AgentContext/add_message
  - functions/src/llm/agent/context/AgentContext/trim_to_fit
---

# Signature

`fn estimate_message_tokens(&self, message: &Message) -> usize`

# Calls

- [estimate_tokens](../../../../../../functions/src/llm/agent/context/AgentContext/estimate_tokens.md)

# Called by

- [add_message](../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [trim_to_fit](../../../../../../functions/src/llm/agent/context/AgentContext/trim_to_fit.md)