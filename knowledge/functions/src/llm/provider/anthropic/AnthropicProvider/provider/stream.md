---
type: Rust Method
title: stream
resource: src/llm/provider/anthropic.rs#L248-L287
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/handle_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/parse_anthropic_sse_stream
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [to_anthropic_request](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/to_anthropic_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [headers](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/headers.md)
- [status](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [handle_error](../../../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/handle_error.md)
- [parse_anthropic_sse_stream](../../../../../../../functions/src/llm/provider/anthropic/parse_anthropic_sse_stream.md)