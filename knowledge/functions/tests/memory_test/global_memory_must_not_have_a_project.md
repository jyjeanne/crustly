---
type: Rust Function
title: global_memory_must_not_have_a_project
resource: tests/memory_test.rs#L82-L100
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
  - target: functions/tests/memory_test/insert_project
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

`async fn global_memory_must_not_have_a_project()`

# Calls

- [migrated_db](../../../functions/tests/memory_test/migrated_db.md)
- [insert_project](../../../functions/tests/memory_test/insert_project.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [insert_memory](../../../functions/tests/memory_test/insert_memory.md)