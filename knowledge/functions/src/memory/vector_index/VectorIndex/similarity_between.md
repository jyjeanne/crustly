---
type: Rust Method
title: similarity_between
resource: src/memory/vector_index.rs#L80-L83
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector/similarity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/semantic/drop_near_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/similarity_between_two_indexed_memories
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn similarity_between(&self, a: &Uuid, b: &Uuid) -> Option<f32>`

# Calls

- [similarity](../../../../../functions/src/memory/vector/similarity.md)

# Called by

- [drop_near_duplicates](../../../../../functions/src/memory/semantic/drop_near_duplicates.md)
- [similarity_between_two_indexed_memories](../../../../../functions/src/memory/vector_index/similarity_between_two_indexed_memories.md)