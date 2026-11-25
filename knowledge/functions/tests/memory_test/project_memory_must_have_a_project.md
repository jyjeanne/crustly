---
type: Rust Function
title: project_memory_must_have_a_project
resource: tests/memory_test.rs#L103-L120
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
  - target: functions/tests/memory_test/insert_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn project_memory_must_have_a_project()`

# Calls

- [migrated_db](../../../functions/tests/memory_test/migrated_db.md)
- [insert_memory](../../../functions/tests/memory_test/insert_memory.md)
- [pool](../../../functions/src/db/Database/pool.md)