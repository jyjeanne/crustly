---
type: Rust Function
title: finish
resource: src/memory/embed.rs#L141-L163
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/embed/OllamaEmbedder/embedder/embed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn finish(vectors: Vec<Vec<f32>>, expected_count: usize) -> Result<Vec<Vec<f32>>, EmbedError>`

# Called by

- [embed](../../../../functions/src/memory/embed/OllamaEmbedder/embedder/embed.md)
- [embed](../../../../functions/src/memory/embed/OpenAiCompatibleEmbedder/embedder/embed.md)