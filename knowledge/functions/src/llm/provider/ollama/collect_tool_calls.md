---
type: Rust Function
title: collect_tool_calls
resource: src/llm/provider/ollama.rs#L785-L790
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/provider/stream
  - functions/src/llm/provider/ollama/streamed_tool_calls_arrive_before_the_done_chunk
---

# Signature

`fn collect_tool_calls(tool_calls: &[ToolCall]) -> Vec<(String, serde_json::Value)>`

# Called by

- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [streamed_tool_calls_arrive_before_the_done_chunk](../../../../../functions/src/llm/provider/ollama/streamed_tool_calls_arrive_before_the_done_chunk.md)