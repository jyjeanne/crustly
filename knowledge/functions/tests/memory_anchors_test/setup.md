---
type: Rust Function
title: setup
resource: tests/memory_anchors_test.rs#L12-L25
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
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn setup() -> (Database, MemoryStore, ProjectIdentity)`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [id_for_root](../../../functions/src/memory/project/id_for_root.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)