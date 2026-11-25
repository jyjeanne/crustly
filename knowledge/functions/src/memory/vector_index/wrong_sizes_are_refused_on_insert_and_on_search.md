---
type: Rust Function
title: wrong_sizes_are_refused_on_insert_and_on_search
resource: src/memory/vector_index.rs#L143-L148
tags:
- test
visibility: private
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
  - target: functions/src/memory/vector_index/unit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/search
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn wrong_sizes_are_refused_on_insert_and_on_search()`

# Calls

- [new](../../../../functions/src/memory/vector_index/VectorIndex/new.md)
- [insert](../../../../functions/src/memory/vector_index/VectorIndex/insert.md)
- [unit](../../../../functions/src/memory/vector_index/unit.md)
- [search](../../../../functions/src/memory/vector_index/VectorIndex/search.md)