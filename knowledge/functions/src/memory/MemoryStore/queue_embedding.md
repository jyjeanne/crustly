---
type: Rust Method
title: queue_embedding
resource: src/memory/mod.rs#L167-L171
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn queue_embedding(&self, id: Uuid)`

# Called by

- [approve](../../../../functions/src/memory/extraction/MemoryStore/approve.md)
- [save_with](../../../../functions/src/memory/MemoryStore/save_with.md)
- [edit](../../../../functions/src/memory/MemoryStore/edit.md)