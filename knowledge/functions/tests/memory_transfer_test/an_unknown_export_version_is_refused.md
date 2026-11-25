---
type: Rust Function
title: an_unknown_export_version_is_refused
resource: tests/memory_transfer_test.rs#L117-L122
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/transfer/export
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_unknown_export_version_is_refused()`

# Calls

- [export](../../../functions/src/memory/transfer/export.md)
- [import](../../../functions/src/memory/transfer/import.md)