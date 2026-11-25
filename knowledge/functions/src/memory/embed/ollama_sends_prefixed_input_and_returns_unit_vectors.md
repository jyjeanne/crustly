---
type: Rust Function
title: ollama_sends_prefixed_input_and_returns_unit_vectors
resource: src/memory/embed.rs#L383-L404
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/embed/OllamaEmbedder/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OllamaEmbedder/embedder/embed
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/OllamaEmbedder/embedder/model_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn ollama_sends_prefixed_input_and_returns_unit_vectors()`

# Calls

- [new](../../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [settings](../../../../functions/src/memory/embed/settings.md)
- [embed](../../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)
- [model_id](../../../../functions/src/memory/embed/OllamaEmbedder/embedder/model_id.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)