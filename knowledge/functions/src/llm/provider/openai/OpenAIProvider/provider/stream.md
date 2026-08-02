---
type: Rust Method
title: stream
resource: src/llm/provider/openai.rs#L599-L883
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/provider/openai/OpenAIProvider/to_openai_request
  - functions/src/llm/provider/retry/retry_with_backoff
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/from_str
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [to_openai_request](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/to_openai_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [from_str](../../../../../../../functions/src/config/secrets/SecretString/from_str.md)