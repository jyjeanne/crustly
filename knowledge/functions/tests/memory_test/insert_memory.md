---
type: Rust Function
title: insert_memory
resource: tests/memory_test.rs#L28-L51
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/memory_test/global_memory_must_not_have_a_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/project_memory_must_have_a_project
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
  - target: functions/tests/memory_test/global_facts_are_unique_across_the_global_scope
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn insert_memory( pool: &SqlitePool, id: &str, scope: &str, project_id: Option<&str>, content: &str, hash: &str, status: &str, ) -> Result<(), sqlx::Error>`

# Called by

- [global_memory_must_not_have_a_project](../../../functions/tests/memory_test/global_memory_must_not_have_a_project.md)
- [project_memory_must_have_a_project](../../../functions/tests/memory_test/project_memory_must_have_a_project.md)
- [fts_index_follows_insert_update_and_delete](../../../functions/tests/memory_test/fts_index_follows_insert_update_and_delete.md)
- [second_active_copy_of_a_fact_is_rejected](../../../functions/tests/memory_test/second_active_copy_of_a_fact_is_rejected.md)
- [superseded_copy_of_a_fact_is_allowed](../../../functions/tests/memory_test/superseded_copy_of_a_fact_is_allowed.md)
- [global_facts_are_unique_across_the_global_scope](../../../functions/tests/memory_test/global_facts_are_unique_across_the_global_scope.md)