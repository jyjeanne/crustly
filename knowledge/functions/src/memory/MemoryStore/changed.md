---
type: Rust Method
title: changed
resource: src/memory/mod.rs#L152-L155
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/reindex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/promote
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/relink
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/add_paths
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn changed(&self)`

# Called by

- [approve](../../../../functions/src/memory/extraction/MemoryStore/approve.md)
- [reindex](../../../../functions/src/memory/MemoryStore/reindex.md)
- [save_with](../../../../functions/src/memory/MemoryStore/save_with.md)
- [forget](../../../../functions/src/memory/MemoryStore/forget.md)
- [edit](../../../../functions/src/memory/MemoryStore/edit.md)
- [promote](../../../../functions/src/memory/MemoryStore/promote.md)
- [relink](../../../../functions/src/memory/MemoryStore/relink.md)
- [add_paths](../../../../functions/src/memory/MemoryStore/add_paths.md)