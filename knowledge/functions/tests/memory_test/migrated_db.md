---
type: Rust Function
title: migrated_db
resource: tests/memory_test.rs#L9-L13
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_test/long_term_memory_tables_exist
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/tests/memory_test/secure_delete_is_on_for_every_connection
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn migrated_db() -> Database`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)

# Called by

- [long_term_memory_tables_exist](../../../functions/tests/memory_test/long_term_memory_tables_exist.md)
- [global_memory_must_not_have_a_project](../../../functions/tests/memory_test/global_memory_must_not_have_a_project.md)
- [project_memory_must_have_a_project](../../../functions/tests/memory_test/project_memory_must_have_a_project.md)
- [fts_index_follows_insert_update_and_delete](../../../functions/tests/memory_test/fts_index_follows_insert_update_and_delete.md)
- [second_active_copy_of_a_fact_is_rejected](../../../functions/tests/memory_test/second_active_copy_of_a_fact_is_rejected.md)
- [superseded_copy_of_a_fact_is_allowed](../../../functions/tests/memory_test/superseded_copy_of_a_fact_is_allowed.md)
- [global_facts_are_unique_across_the_global_scope](../../../functions/tests/memory_test/global_facts_are_unique_across_the_global_scope.md)
- [secure_delete_is_on_for_every_connection](../../../functions/tests/memory_test/secure_delete_is_on_for_every_connection.md)