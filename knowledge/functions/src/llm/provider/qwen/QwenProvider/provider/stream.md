---
type: Rust Method
title: stream
resource: src/llm/provider/qwen.rs#L1346-L1523
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/to_qwen_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/retry/retry_with_backoff
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/headers
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/PullProgress/is_success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/handle_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/llm_response_to_stream_events
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [to_qwen_request](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/to_qwen_request.md)
- [retry_with_backoff](../../../../../../../functions/src/llm/provider/retry/retry_with_backoff.md)
- [headers](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/headers.md)
- [status](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)
- [is_success](../../../../../../../functions/src/llm/provider/ollama_models/PullProgress/is_success.md)
- [handle_error](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/handle_error.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [from_qwen_response](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [llm_response_to_stream_events](../../../../../../../functions/src/llm/provider/qwen/llm_response_to_stream_events.md)

# Called by

- [stream_assembles_openai_style_tool_call_across_sse_chunks](../../../../../../../functions/src/llm/provider/qwen/stream_assembles_openai_style_tool_call_across_sse_chunks.md)
- [stream_skips_malformed_sse_chunk_and_continues](../../../../../../../functions/src/llm/provider/qwen/stream_skips_malformed_sse_chunk_and_continues.md)