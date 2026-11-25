---
type: Rust Function
title: collect_tool_calls
resource: src/llm/provider/ollama.rs#L685-L690
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaStreamState/on_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/streamed_tool_calls_arrive_before_the_done_chunk
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn collect_tool_calls(tool_calls: &[ToolCall]) -> Vec<(String, serde_json::Value)>`

# Called by

- [on_chunk](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)
- [streamed_tool_calls_arrive_before_the_done_chunk](../../../../../functions/src/llm/provider/ollama/streamed_tool_calls_arrive_before_the_done_chunk.md)