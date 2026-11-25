---
type: Rust Function
title: maybe_tool_call_json
resource: src/llm/provider/tool_call_recovery.rs#L25-L28
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/run_stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/on_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/only_json_like_content_is_withheld_from_streaming
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn maybe_tool_call_json(text: &str) -> bool`

# Called by

- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)
- [on_chunk](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)
- [only_json_like_content_is_withheld_from_streaming](../../../../../functions/src/llm/provider/tool_call_recovery/only_json_like_content_is_withheld_from_streaming.md)