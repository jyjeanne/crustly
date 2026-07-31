---
type: Rust Method
title: complete
resource: src/llm/provider/gemini.rs#L542-L591
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
  - functions/src/llm/provider/gemini/GeminiProvider/generate_url
  - functions/src/llm/provider/retry/retry_with_backoff
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [to_gemini_request](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [generate_url](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/generate_url.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [from_gemini_response](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/from_gemini_response.md)