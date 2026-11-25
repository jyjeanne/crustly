---
type: Rust Method
title: insert
resource: src/memory/vector_index.rs#L64-L73
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/vector_index/VectorIndex/from_entries
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

`pub fn insert(&mut self, id: Uuid, vector: Vec<f32>) -> Result<(), VectorError>`

# Called by

- [from_entries](../../../../../functions/src/memory/vector_index/VectorIndex/from_entries.md)
- [wrong_sizes_are_refused_on_insert_and_on_search](../../../../../functions/src/memory/vector_index/wrong_sizes_are_refused_on_insert_and_on_search.md)
- [insert_replaces_and_remove_deletes](../../../../../functions/src/memory/vector_index/insert_replaces_and_remove_deletes.md)