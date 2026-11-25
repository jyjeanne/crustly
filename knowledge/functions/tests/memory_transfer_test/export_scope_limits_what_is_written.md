---
type: Rust Function
title: export_scope_limits_what_is_written
resource: tests/memory_transfer_test.rs#L91-L114
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_transfer_test/project_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/export
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn export_scope_limits_what_is_written()`

# Calls

- [project_at](../../../functions/tests/memory_transfer_test/project_at.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [export](../../../functions/src/memory/transfer/export.md)