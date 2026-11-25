---
type: Rust Function
title: global_facts_are_unique_across_the_global_scope
resource: tests/memory_test.rs#L235-L264
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
  - target: functions/tests/memory_test/insert_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn global_facts_are_unique_across_the_global_scope()`

# Calls

- [migrated_db](../../../functions/tests/memory_test/migrated_db.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [insert_memory](../../../functions/tests/memory_test/insert_memory.md)