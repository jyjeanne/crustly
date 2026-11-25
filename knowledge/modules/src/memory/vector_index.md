---
type: Rust Module
title: vector_index
resource: src/memory/vector_index.rs#L1-L171
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-vector-similarity-vectorerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-vector-normalize
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [VectorIndex](../../../classes/src/memory/vector_index/VectorIndex.md)
- [new](../../../functions/src/memory/vector_index/VectorIndex/new.md)
- [from_entries](../../../functions/src/memory/vector_index/VectorIndex/from_entries.md)
- [model](../../../functions/src/memory/vector_index/VectorIndex/model.md)
- [dimensions](../../../functions/src/memory/vector_index/VectorIndex/dimensions.md)
- [len](../../../functions/src/memory/vector_index/VectorIndex/len.md)
- [is_empty](../../../functions/src/memory/vector_index/VectorIndex/is_empty.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [insert](../../../functions/src/memory/vector_index/VectorIndex/insert.md)
- [remove](../../../functions/src/memory/vector_index/VectorIndex/remove.md)
- [similarity_between](../../../functions/src/memory/vector_index/VectorIndex/similarity_between.md)
- [search](../../../functions/src/memory/vector_index/VectorIndex/search.md)
- [unit](../../../functions/src/memory/vector_index/unit.md)
- [search_returns_the_most_similar_first_and_respects_k](../../../functions/src/memory/vector_index/search_returns_the_most_similar_first_and_respects_k.md)
- [search_drops_hits_below_the_threshold](../../../functions/src/memory/vector_index/search_drops_hits_below_the_threshold.md)
- [wrong_sizes_are_refused_on_insert_and_on_search](../../../functions/src/memory/vector_index/wrong_sizes_are_refused_on_insert_and_on_search.md)
- [insert_replaces_and_remove_deletes](../../../functions/src/memory/vector_index/insert_replaces_and_remove_deletes.md)
- [similarity_between_two_indexed_memories](../../../functions/src/memory/vector_index/similarity_between_two_indexed_memories.md)

# Imports

- `super::vector::{similarity, VectorError}`
- `std::collections::HashMap`
- `uuid::Uuid`
- `super::*`
- `crate::memory::vector::normalize`

# Member of

- [crustly](../../../packages/crustly.md)