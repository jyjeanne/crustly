---
type: Rust Function
title: export_then_import_recreates_the_memories_in_a_new_database
resource: tests/memory_transfer_test.rs#L50-L88
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/memory/transfer/import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn export_then_import_recreates_the_memories_in_a_new_database()`

# Calls

- [project_at](../../../functions/tests/memory_transfer_test/project_at.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [export](../../../functions/src/memory/transfer/export.md)
- [import](../../../functions/src/memory/transfer/import.md)
- [all](../../../functions/src/memory/MemoryStore/all.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)