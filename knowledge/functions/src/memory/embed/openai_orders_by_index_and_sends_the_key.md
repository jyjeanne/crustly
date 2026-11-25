---
type: Rust Function
title: openai_orders_by_index_and_sends_the_key
resource: src/memory/embed.rs#L433-L457
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/embed/settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/model_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn openai_orders_by_index_and_sends_the_key()`

# Calls

- [settings](../../../../functions/src/memory/embed/settings.md)
- [new](../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/new.md)
- [embed](../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed.md)
- [model_id](../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/model_id.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)