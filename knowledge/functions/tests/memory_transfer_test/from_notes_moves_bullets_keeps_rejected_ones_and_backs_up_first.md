---
type: Rust Function
title: from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first
resource: tests/memory_transfer_test.rs#L125-L172
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first()`

# Calls

- [write](../../../functions/src/memory/project/write.md)
- [identify](../../../functions/src/memory/project/identify.md)
- [import_from_notes](../../../functions/src/memory/transfer/import_from_notes.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)