---
type: Rust Method
title: complete
resource: src/llm/provider/anthropic.rs#L191-L246
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/to_anthropic_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/retry/retry_with_backoff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/headers
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/handle_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/from_anthropic_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [to_anthropic_request](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/to_anthropic_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [headers](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/headers.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [handle_error](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/handle_error.md)
- [from_anthropic_response](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/from_anthropic_response.md)