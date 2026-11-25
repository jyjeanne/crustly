---
type: Rust Function
title: usage_comes_from_the_done_chunk
resource: src/llm/provider/ollama.rs#L1792-L1815
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
---

# Signature

`fn usage_comes_from_the_done_chunk()`

# Calls

- [new](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/new.md)
- [on_chunk](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)
- [chunk](../../../../../functions/src/llm/provider/ollama/chunk.md)
- [finish](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/finish.md)