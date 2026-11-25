---
type: Rust Function
title: fts_hits
resource: tests/memory_test.rs#L53-L59
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/memory_test/fts_index_follows_insert_update_and_delete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn fts_hits(pool: &SqlitePool, word: &str) -> i64`

# Called by

- [fts_index_follows_insert_update_and_delete](../../../functions/tests/memory_test/fts_index_follows_insert_update_and_delete.md)