---
type: Rust Function
title: an_unrelated_checksum_mismatch_is_still_refused
resource: tests/memory_migration_test.rs#L122-L139
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
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/close
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_unrelated_checksum_mismatch_is_still_refused()`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [close](../../../functions/src/db/Database/close.md)