---
type: Rust Method
title: stream
resource: src/llm/provider/gemini.rs#L599-L647
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/stream_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/retry/retry_with_backoff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/headers
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/GeminiProvider/handle_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/parse_gemini_sse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [to_gemini_request](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/to_gemini_request.md)
- [stream_url](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/stream_url.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [headers](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/headers.md)
- [status](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [handle_error](../../../../../../../functions/src/llm/provider/gemini/GeminiProvider/handle_error.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [parse_gemini_sse](../../../../../../../functions/src/llm/provider/gemini/parse_gemini_sse.md)