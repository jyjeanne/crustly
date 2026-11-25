---
type: Rust Function
title: pair_similarity
resource: src/memory/consolidate.rs#L49-L51
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector/similarity
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pair_similarity(a: &[f32], b: &[f32]) -> Option<f32>`

# Calls

- [similarity](../../../../functions/src/memory/vector/similarity.md)

# Called by

- [consolidate](../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)