---
type: Rust Method
title: open_text_block
resource: src/llm/provider/ollama.rs#L742-L752
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaStreamState/on_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/finish
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn open_text_block(&mut self, events: &mut Vec<StreamEvent>)`

# Called by

- [on_chunk](../../../../../../functions/src/llm/provider/ollama/OllamaStreamState/on_chunk.md)
- [finish](../../../../../../functions/src/llm/provider/ollama/OllamaStreamState/finish.md)