---
type: Rust Function
title: ollama_reports_an_error_status_without_panicking
resource: src/memory/embed.rs#L407-L418
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`async fn ollama_reports_an_error_status_without_panicking()`

# Calls

- [new](../../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [settings](../../../../functions/src/memory/embed/settings.md)
- [embed](../../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)