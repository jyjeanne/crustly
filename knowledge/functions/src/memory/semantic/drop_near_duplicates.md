---
type: Rust Function
title: drop_near_duplicates
resource: src/memory/semantic.rs#L253-L266
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/similarity_between
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn drop_near_duplicates(selected: Vec<Memory>, index: &VectorIndex) -> Vec<Memory>`

# Calls

- [similarity_between](../../../../functions/src/memory/vector_index/VectorIndex/similarity_between.md)

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)