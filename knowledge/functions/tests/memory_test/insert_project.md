---
type: Rust Function
title: insert_project
resource: tests/memory_test.rs#L15-L24
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/memory_test/global_memory_must_not_have_a_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/fts_index_follows_insert_update_and_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/second_active_copy_of_a_fact_is_rejected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/superseded_copy_of_a_fact_is_allowed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn insert_project(pool: &SqlitePool, id: &str)`

# Called by

- [global_memory_must_not_have_a_project](../../../functions/tests/memory_test/global_memory_must_not_have_a_project.md)
- [fts_index_follows_insert_update_and_delete](../../../functions/tests/memory_test/fts_index_follows_insert_update_and_delete.md)
- [second_active_copy_of_a_fact_is_rejected](../../../functions/tests/memory_test/second_active_copy_of_a_fact_is_rejected.md)
- [superseded_copy_of_a_fact_is_allowed](../../../functions/tests/memory_test/superseded_copy_of_a_fact_is_allowed.md)