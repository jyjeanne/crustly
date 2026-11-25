---
type: Rust Function
title: ollama_refuses_a_wrong_number_of_vectors
resource: src/memory/embed.rs#L421-L430
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
---

# Signature

`async fn ollama_refuses_a_wrong_number_of_vectors()`

# Calls

- [new](../../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [settings](../../../../functions/src/memory/embed/settings.md)
- [embed](../../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)