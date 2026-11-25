---
type: Rust Function
title: long_term_memory_tables_exist
resource: tests/memory_test.rs#L62-L79
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_test/migrated_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn long_term_memory_tables_exist()`

# Calls

- [migrated_db](../../../functions/tests/memory_test/migrated_db.md)
- [pool](../../../functions/src/db/Database/pool.md)