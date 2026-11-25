---
type: Rust Function
title: repo
resource: tests/memory_review_fixes_test.rs#L33-L40
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/upsert_project
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn repo() -> (Database, MemoryRepository)`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [upsert_project](../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)