---
type: Rust Function
title: json_after_text_is_streamed_as_text_and_never_run
resource: src/llm/provider/ollama.rs#L1887-L1912
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaStreamState/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaStreamState/on_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/finish
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/read_file_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/stop_reason
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn json_after_text_is_streamed_as_text_and_never_run()`

# Calls

- [new](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/new.md)
- [on_chunk](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)
- [chunk](../../../../../functions/src/llm/provider/ollama/chunk.md)
- [finish](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/finish.md)
- [read_file_tool](../../../../../functions/src/llm/provider/ollama/read_file_tool.md)
- [stop_reason](../../../../../functions/src/llm/provider/ollama/stop_reason.md)