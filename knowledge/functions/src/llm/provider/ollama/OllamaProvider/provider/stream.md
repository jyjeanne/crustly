---
type: Rust Method
title: stream
resource: src/llm/provider/ollama.rs#L549-L613
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/on_chunk
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>`

# Calls

- [to_ollama_request](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [new](../../../../../../../functions/src/llm/provider/ollama/OllamaStreamState/new.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [on_chunk](../../../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)

# Called by

- [streamed_ollama_tool_call_survives_drain](../../../../../../../functions/src/llm/agent/service/streamed_ollama_tool_call_survives_drain.md)
- [streamed_tool_call_reaches_caller](../../../../../../../functions/src/llm/provider/ollama/streamed_tool_call_reaches_caller.md)
- [text_arrives_before_the_response_is_complete](../../../../../../../functions/src/llm/provider/ollama/text_arrives_before_the_response_is_complete.md)
- [a_reply_cut_off_before_the_done_chunk_is_an_error](../../../../../../../functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error.md)