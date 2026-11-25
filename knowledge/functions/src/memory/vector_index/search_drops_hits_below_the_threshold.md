---
type: Rust Function
title: search_drops_hits_below_the_threshold
resource: src/memory/vector_index.rs#L133-L140
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/from_entries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/search
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/unit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn search_drops_hits_below_the_threshold()`

# Calls

- [from_entries](../../../../functions/src/memory/vector_index/VectorIndex/from_entries.md)
- [search](../../../../functions/src/memory/vector_index/VectorIndex/search.md)
- [unit](../../../../functions/src/memory/vector_index/unit.md)