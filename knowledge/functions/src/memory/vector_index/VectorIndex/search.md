---
type: Rust Method
title: search
resource: src/memory/vector_index.rs#L88-L101
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector/similarity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/vector_index/search_returns_the_most_similar_first_and_respects_k
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/search_drops_hits_below_the_threshold
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/wrong_sizes_are_refused_on_insert_and_on_search
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/insert_replaces_and_remove_deletes
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn search(&self, query: &[f32], k: usize, min_similarity: f32) -> Vec<(Uuid, f32)>`

# Calls

- [similarity](../../../../../functions/src/memory/vector/similarity.md)

# Called by

- [search_returns_the_most_similar_first_and_respects_k](../../../../../functions/src/memory/vector_index/search_returns_the_most_similar_first_and_respects_k.md)
- [search_drops_hits_below_the_threshold](../../../../../functions/src/memory/vector_index/search_drops_hits_below_the_threshold.md)
- [wrong_sizes_are_refused_on_insert_and_on_search](../../../../../functions/src/memory/vector_index/wrong_sizes_are_refused_on_insert_and_on_search.md)
- [insert_replaces_and_remove_deletes](../../../../../functions/src/memory/vector_index/insert_replaces_and_remove_deletes.md)