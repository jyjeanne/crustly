---
type: Rust Method
title: extract_thinking_from_response
resource: src/llm/agent/service.rs#L1720-L1735
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/agent/service/AgentService/final_text_and_thinking
---

# Signature

`fn extract_thinking_from_response(response: &LLMResponse) -> Option<String>`

# Calls

- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [final_text_and_thinking](../../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)