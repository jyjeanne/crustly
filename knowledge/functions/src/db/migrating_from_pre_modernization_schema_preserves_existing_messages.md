---
type: Rust Function
title: migrating_from_pre_modernization_schema_preserves_existing_messages
resource: src/db/mod.rs#L464-L534
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn migrating_from_pre_modernization_schema_preserves_existing_messages()`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [write](../../../functions/src/memory/project/write.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)