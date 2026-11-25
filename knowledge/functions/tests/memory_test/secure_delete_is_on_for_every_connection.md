---
type: Rust Function
title: secure_delete_is_on_for_every_connection
resource: tests/memory_test.rs#L267-L281
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_test/migrated_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/task/FileLock/acquire
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn secure_delete_is_on_for_every_connection()`

# Calls

- [migrated_db](../../../functions/tests/memory_test/migrated_db.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [acquire](../../../functions/src/llm/tools/task/FileLock/acquire.md)