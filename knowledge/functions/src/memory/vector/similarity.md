---
type: Rust Function
title: similarity
resource: src/memory/vector.rs#L87-L95
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/consolidate/pair_similarity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/archive_near_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector/similarity_is_the_dot_product_of_unit_vectors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector/similarity_refuses_different_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/similarity_between
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/search
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn similarity(a: &[f32], b: &[f32]) -> Result<f32, VectorError>`

# Called by

- [pair_similarity](../../../../functions/src/memory/consolidate/pair_similarity.md)
- [archive_near_duplicates](../../../../functions/src/memory/semantic/archive_near_duplicates.md)
- [similarity_is_the_dot_product_of_unit_vectors](../../../../functions/src/memory/vector/similarity_is_the_dot_product_of_unit_vectors.md)
- [similarity_refuses_different_dimensions](../../../../functions/src/memory/vector/similarity_refuses_different_dimensions.md)
- [similarity_between](../../../../functions/src/memory/vector_index/VectorIndex/similarity_between.md)
- [search](../../../../functions/src/memory/vector_index/VectorIndex/search.md)