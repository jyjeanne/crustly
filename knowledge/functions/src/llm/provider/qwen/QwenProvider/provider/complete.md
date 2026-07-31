---
type: Rust Method
title: complete
resource: src/llm/provider/qwen.rs#L1274-L1323
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/provider/retry/retry_with_backoff
  - functions/src/llm/provider/ollama_models/PullProgress/is_success
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [to_qwen_request](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [from_qwen_response](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)