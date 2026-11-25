---
type: Rust Function
title: bench_keyword_recall
resource: benches/memory_recall.rs#L43-L94
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
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn bench_keyword_recall(c: &mut Criterion)`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [identify](../../../functions/src/memory/project/identify.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [save](../../../functions/src/memory/MemoryStore/save.md)
- [recall](../../../functions/src/memory/MemoryStore/recall.md)