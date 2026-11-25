---
type: Rust Function
title: unit
resource: src/memory/vector_index.rs#L109-L111
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/vector_index/search_returns_the_most_similar_first_and_respects_k
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/search_drops_hits_below_the_threshold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/wrong_sizes_are_refused_on_insert_and_on_search
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/insert_replaces_and_remove_deletes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/similarity_between_two_indexed_memories
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn unit(v: &[f32]) -> Vec<f32>`

# Called by

- [search_returns_the_most_similar_first_and_respects_k](../../../../functions/src/memory/vector_index/search_returns_the_most_similar_first_and_respects_k.md)
- [search_drops_hits_below_the_threshold](../../../../functions/src/memory/vector_index/search_drops_hits_below_the_threshold.md)
- [wrong_sizes_are_refused_on_insert_and_on_search](../../../../functions/src/memory/vector_index/wrong_sizes_are_refused_on_insert_and_on_search.md)
- [insert_replaces_and_remove_deletes](../../../../functions/src/memory/vector_index/insert_replaces_and_remove_deletes.md)
- [similarity_between_two_indexed_memories](../../../../functions/src/memory/vector_index/similarity_between_two_indexed_memories.md)