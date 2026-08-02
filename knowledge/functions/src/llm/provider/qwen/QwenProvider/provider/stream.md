---
type: Rust Method
title: stream
resource: src/llm/provider/qwen.rs#L1325-L1502
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
  - functions/src/llm/provider/retry/retry_with_backoff
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
  - functions/src/llm/provider/qwen/llm_response_to_stream_events
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [to_qwen_request](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [from_qwen_response](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [llm_response_to_stream_events](../../../../../../../functions/src/llm/provider/qwen/llm_response_to_stream_events.md)