---
type: Rust Module
title: memory_test
resource: tests/memory_test.rs#L1-L281
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sqlx-sqlitepool
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [migrated_db](../../functions/tests/memory_test/migrated_db.md)
- [insert_project](../../functions/tests/memory_test/insert_project.md)
- [insert_memory](../../functions/tests/memory_test/insert_memory.md)
- [fts_hits](../../functions/tests/memory_test/fts_hits.md)
- [long_term_memory_tables_exist](../../functions/tests/memory_test/long_term_memory_tables_exist.md)
- [global_memory_must_not_have_a_project](../../functions/tests/memory_test/global_memory_must_not_have_a_project.md)
- [project_memory_must_have_a_project](../../functions/tests/memory_test/project_memory_must_have_a_project.md)
- [fts_index_follows_insert_update_and_delete](../../functions/tests/memory_test/fts_index_follows_insert_update_and_delete.md)
- [second_active_copy_of_a_fact_is_rejected](../../functions/tests/memory_test/second_active_copy_of_a_fact_is_rejected.md)
- [superseded_copy_of_a_fact_is_allowed](../../functions/tests/memory_test/superseded_copy_of_a_fact_is_allowed.md)
- [global_facts_are_unique_across_the_global_scope](../../functions/tests/memory_test/global_facts_are_unique_across_the_global_scope.md)
- [secure_delete_is_on_for_every_connection](../../functions/tests/memory_test/secure_delete_is_on_for_every_connection.md)

# Imports

- `crustly::db::Database`
- `sqlx::SqlitePool`

# Member of

- [crustly](../../packages/crustly.md)