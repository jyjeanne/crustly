---
type: Rust Function
title: a_checksum_recorded_with_other_line_endings_is_accepted_and_updated
resource: tests/memory_migration_test.rs#L77-L117
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/close
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_checksum_recorded_with_other_line_endings_is_accepted_and_updated()`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [find](../../../functions/src/memory/MemoryStore/find.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [close](../../../functions/src/db/Database/close.md)