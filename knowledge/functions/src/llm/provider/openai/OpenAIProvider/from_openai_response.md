---
type: Rust Method
title: from_openai_response
resource: src/llm/provider/openai.rs#L348-L470
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/types/extract_think_tags
  - functions/src/config/secrets/SecretString/from_str
  called_by:
  - functions/src/llm/provider/openai/OpenAIProvider/provider/complete
---

# Signature

`fn from_openai_response(&self, response: OpenAIResponse) -> LLMResponse`

# Calls

- [next](../../../../../../functions/src/tui/events/EventHandler/next.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [extract_think_tags](../../../../../../functions/src/llm/provider/types/extract_think_tags.md)
- [from_str](../../../../../../functions/src/config/secrets/SecretString/from_str.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)