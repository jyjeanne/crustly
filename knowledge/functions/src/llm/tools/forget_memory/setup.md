---
type: Rust Function
title: setup
resource: src/llm/tools/forget_memory.rs#L174-L179
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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
---

# Signature

`async fn setup() -> (Database, Arc<MemoryStore>)`

# Calls

- [connect_in_memory](../../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../../../functions/src/db/Database/pool.md)
- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)