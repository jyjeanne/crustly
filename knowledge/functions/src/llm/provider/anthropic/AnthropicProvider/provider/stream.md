---
type: Rust Method
title: stream
resource: src/llm/provider/anthropic.rs#L248-L287
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/provider/anthropic/AnthropicProvider/to_anthropic_request
  - functions/src/llm/provider/retry/retry_with_backoff
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/llm/provider/anthropic/parse_anthropic_sse_stream
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [to_anthropic_request](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/to_anthropic_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [parse_anthropic_sse_stream](../../../../../../../functions/src/llm/provider/anthropic/parse_anthropic_sse_stream.md)