---
type: Rust Function
title: create_pool_with_schema
resource: tests/codebase_index_test.rs#L8-L24
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/tests/codebase_index_test/index_and_query_provider_trait
  - functions/tests/codebase_index_test/index_file_twice_no_duplicate
  - functions/tests/codebase_index_test/fts_search_finds_symbol_by_partial_name
  - functions/tests/codebase_index_test/index_nonexistent_file_returns_error
---

# Signature

`async fn create_pool_with_schema() -> sqlx::SqlitePool`

# Called by

- [index_and_query_provider_trait](../../../functions/tests/codebase_index_test/index_and_query_provider_trait.md)
- [index_file_twice_no_duplicate](../../../functions/tests/codebase_index_test/index_file_twice_no_duplicate.md)
- [fts_search_finds_symbol_by_partial_name](../../../functions/tests/codebase_index_test/fts_search_finds_symbol_by_partial_name.md)
- [index_nonexistent_file_returns_error](../../../functions/tests/codebase_index_test/index_nonexistent_file_returns_error.md)