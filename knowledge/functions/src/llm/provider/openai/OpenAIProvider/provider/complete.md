---
type: Rust Method
title: complete
resource: src/llm/provider/openai.rs#L547-L609
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/to_openai_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/retry/retry_with_backoff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/headers
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/handle_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/from_openai_response
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/provider/complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [to_openai_request](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/to_openai_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [headers](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/headers.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [handle_error](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/handle_error.md)
- [from_openai_response](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/from_openai_response.md)

# Called by

- [complete](../../../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/complete.md)