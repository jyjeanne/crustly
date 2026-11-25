---
type: Rust Method
title: stream
resource: src/llm/provider/openai.rs#L611-L681
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/handle_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIStreamState/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIStreamState/on_bytes
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/azure/AzureOpenAIProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [to_openai_request](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/to_openai_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [headers](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/headers.md)
- [status](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [handle_error](../../../../../../../functions/src/llm/provider/openai/OpenAIProvider/handle_error.md)
- [new](../../../../../../../functions/src/llm/provider/openai/OpenAIStreamState/new.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [on_bytes](../../../../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_bytes.md)

# Called by

- [stream](../../../../../../../functions/src/llm/provider/azure/AzureOpenAIProvider/provider/stream.md)
- [text_arrives_before_the_response_is_complete](../../../../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)