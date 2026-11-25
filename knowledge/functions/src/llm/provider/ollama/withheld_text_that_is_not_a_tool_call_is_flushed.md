---
type: Rust Function
title: withheld_text_that_is_not_a_tool_call_is_flushed
resource: src/llm/provider/ollama.rs#L1781-L1789
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaStreamState/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaStreamState/finish
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/read_file_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/on_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/stop_reason
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn withheld_text_that_is_not_a_tool_call_is_flushed()`

# Calls

- [new](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/new.md)
- [finish](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/finish.md)
- [read_file_tool](../../../../../functions/src/llm/provider/ollama/read_file_tool.md)
- [on_chunk](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)
- [chunk](../../../../../functions/src/llm/provider/ollama/chunk.md)
- [stop_reason](../../../../../functions/src/llm/provider/ollama/stop_reason.md)