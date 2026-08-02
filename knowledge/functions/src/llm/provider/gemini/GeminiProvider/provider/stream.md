---
type: Rust Method
title: stream
resource: src/llm/provider/gemini.rs#L593-L641
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
  - functions/src/llm/provider/gemini/GeminiProvider/stream_url
  - functions/src/llm/provider/retry/retry_with_backoff
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/tui/events/EventHandler/next
  - functions/src/llm/provider/gemini/parse_gemini_sse
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [to_gemini_request](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [stream_url](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/stream_url.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [parse_gemini_sse](../../../../../../../functions/src/llm/provider/gemini/parse_gemini_sse.md)