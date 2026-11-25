---
type: Rust Function
title: index_of
resource: benches/memory_recall.rs#L26-L30
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/benches/memory_recall/vector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/from_entries
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/memory_recall/bench_vector_search
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn index_of(size: usize) -> (VectorIndex, Vec<f32>)`

# Calls

- [vector](../../../functions/benches/memory_recall/vector.md)
- [from_entries](../../../functions/src/memory/vector_index/VectorIndex/from_entries.md)

# Called by

- [bench_vector_search](../../../functions/benches/memory_recall/bench_vector_search.md)