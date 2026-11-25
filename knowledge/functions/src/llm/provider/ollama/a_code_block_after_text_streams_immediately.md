---
type: Rust Function
title: a_code_block_after_text_streams_immediately
resource: src/llm/provider/ollama.rs#L1917-L1925
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
---

# Signature

`fn a_code_block_after_text_streams_immediately()`

# Calls

- [new](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/new.md)
- [on_chunk](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)
- [chunk](../../../../../functions/src/llm/provider/ollama/chunk.md)