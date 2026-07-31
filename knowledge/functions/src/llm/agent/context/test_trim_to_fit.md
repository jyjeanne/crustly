---
type: Rust Function
title: test_trim_to_fit
resource: src/llm/agent/context.rs#L282-L298
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/types/Message/user
  - functions/src/llm/agent/context/AgentContext/add_message
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/agent/context/AgentContext/trim_to_fit
---

# Signature

`fn test_trim_to_fit()`

# Calls

- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [trim_to_fit](../../../../../functions/src/llm/agent/context/AgentContext/trim_to_fit.md)