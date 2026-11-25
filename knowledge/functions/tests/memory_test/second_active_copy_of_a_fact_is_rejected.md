---
type: Rust Function
title: second_active_copy_of_a_fact_is_rejected
resource: tests/memory_test.rs#L172-L202
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
  - target: functions/tests/memory_test/insert_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/insert_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn second_active_copy_of_a_fact_is_rejected()`

# Calls

- [migrated_db](../../../functions/tests/memory_test/migrated_db.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [insert_project](../../../functions/tests/memory_test/insert_project.md)
- [insert_memory](../../../functions/tests/memory_test/insert_memory.md)