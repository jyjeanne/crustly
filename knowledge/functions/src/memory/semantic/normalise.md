---
type: Rust Function
title: normalise
resource: src/memory/semantic.rs#L246-L249
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/semantic_scores
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn normalise(cosine: f32, min_similarity: f32) -> f64`

# Called by

- [semantic_scores](../../../../functions/src/memory/MemoryStore/semantic_scores.md)