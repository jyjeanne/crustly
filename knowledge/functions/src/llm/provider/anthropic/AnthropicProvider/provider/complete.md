---
type: Rust Method
title: complete
resource: src/llm/provider/anthropic.rs#L191-L246
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/provider/anthropic/AnthropicProvider/to_anthropic_request
  - functions/src/llm/provider/retry/retry_with_backoff
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/llm/provider/anthropic/AnthropicProvider/from_anthropic_response
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [to_anthropic_request](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/to_anthropic_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [from_anthropic_response](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/from_anthropic_response.md)