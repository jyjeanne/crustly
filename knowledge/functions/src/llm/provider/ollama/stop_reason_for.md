---
type: Rust Function
title: stop_reason_for
resource: src/llm/provider/ollama.rs#L693-L699
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaStreamState/finish
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/streamed_tool_calls_arrive_before_the_done_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/stream_without_tool_calls_ends_the_turn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stop_reason_for(tool_calls: &[(String, serde_json::Value)]) -> StopReason`

# Called by

- [finish](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/finish.md)
- [streamed_tool_calls_arrive_before_the_done_chunk](../../../../../functions/src/llm/provider/ollama/streamed_tool_calls_arrive_before_the_done_chunk.md)
- [stream_without_tool_calls_ends_the_turn](../../../../../functions/src/llm/provider/ollama/stream_without_tool_calls_ends_the_turn.md)