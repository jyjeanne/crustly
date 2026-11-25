---
type: Rust Function
title: fts_index_follows_insert_update_and_delete
resource: tests/memory_test.rs#L123-L169
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
  - target: functions/tests/memory_test/insert_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/insert_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/fts_hits
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn fts_index_follows_insert_update_and_delete()`

# Calls

- [migrated_db](../../../functions/tests/memory_test/migrated_db.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [insert_project](../../../functions/tests/memory_test/insert_project.md)
- [insert_memory](../../../functions/tests/memory_test/insert_memory.md)
- [fts_hits](../../../functions/tests/memory_test/fts_hits.md)