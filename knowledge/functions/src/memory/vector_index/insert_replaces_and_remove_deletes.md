---
type: Rust Function
title: insert_replaces_and_remove_deletes
resource: src/memory/vector_index.rs#L151-L160
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/memory/vector_index/VectorIndex/remove
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/len
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/search
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/is_empty
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn insert_replaces_and_remove_deletes()`

# Calls

- [new](../../../../functions/src/memory/vector_index/VectorIndex/new.md)
- [insert](../../../../functions/src/memory/vector_index/VectorIndex/insert.md)
- [unit](../../../../functions/src/memory/vector_index/unit.md)
- [remove](../../../../functions/src/memory/vector_index/VectorIndex/remove.md)
- [len](../../../../functions/src/memory/vector_index/VectorIndex/len.md)
- [search](../../../../functions/src/memory/vector_index/VectorIndex/search.md)
- [is_empty](../../../../functions/src/memory/vector_index/VectorIndex/is_empty.md)