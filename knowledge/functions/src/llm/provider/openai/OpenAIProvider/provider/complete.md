---
type: Rust Method
title: complete
resource: src/llm/provider/openai.rs#L535-L597
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/provider/openai/OpenAIProvider/to_openai_request
  - functions/src/llm/provider/retry/retry_with_backoff
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/llm/provider/openai/OpenAIProvider/from_openai_response
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [to_openai_request](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/to_openai_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [from_openai_response](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/from_openai_response.md)