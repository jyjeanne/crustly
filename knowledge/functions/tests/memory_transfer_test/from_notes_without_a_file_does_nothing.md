---
type: Rust Function
title: from_notes_without_a_file_does_nothing
resource: tests/memory_transfer_test.rs#L175-L189
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn from_notes_without_a_file_does_nothing()`

# Calls

- [identify](../../../functions/src/memory/project/identify.md)
- [import_from_notes](../../../functions/src/memory/transfer/import_from_notes.md)