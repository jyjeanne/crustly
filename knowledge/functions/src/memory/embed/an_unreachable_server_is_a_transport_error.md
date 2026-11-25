---
type: Rust Function
title: an_unreachable_server_is_a_transport_error
resource: src/memory/embed.rs#L460-L469
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

`async fn an_unreachable_server_is_a_transport_error()`

# Calls

- [new](../../../../functions/src/memory/embed/OllamaEmbedder/new.md)
- [settings](../../../../functions/src/memory/embed/settings.md)
- [embed](../../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)