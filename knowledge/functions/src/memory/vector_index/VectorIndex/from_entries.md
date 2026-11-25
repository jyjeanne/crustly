---
type: Rust Method
title: from_entries
resource: src/memory/vector_index.rs#L31-L41
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/benches/memory_recall/index_of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/current_index
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/search_returns_the_most_similar_first_and_respects_k
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/search_drops_hits_below_the_threshold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/similarity_between_two_indexed_memories
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn from_entries( model: &str, dimensions: usize, entries: Vec<(Uuid, Vec<f32>)>, ) -> Result<Self, VectorError>`

# Calls

- [new](../../../../../functions/src/memory/vector_index/VectorIndex/new.md)
- [insert](../../../../../functions/src/memory/vector_index/VectorIndex/insert.md)

# Called by

- [index_of](../../../../../functions/benches/memory_recall/index_of.md)
- [current_index](../../../../../functions/src/memory/semantic/current_index.md)
- [search_returns_the_most_similar_first_and_respects_k](../../../../../functions/src/memory/vector_index/search_returns_the_most_similar_first_and_respects_k.md)
- [search_drops_hits_below_the_threshold](../../../../../functions/src/memory/vector_index/search_drops_hits_below_the_threshold.md)
- [similarity_between_two_indexed_memories](../../../../../functions/src/memory/vector_index/similarity_between_two_indexed_memories.md)