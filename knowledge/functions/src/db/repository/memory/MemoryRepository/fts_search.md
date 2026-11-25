---
type: Rust Method
title: fts_search
resource: src/db/repository/memory.rs#L611-L637
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/row_to_memory
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/search
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/fts_search_finds_the_current_words_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/fts_search_never_crosses_projects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/deleted_memory_is_gone_from_storage_and_search
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn fts_search( &self, match_expr: &str, project_id: &ProjectId, now: DateTime<Utc>, limit: i64, ) -> Result<Vec<(Memory, f64)>>`

# Calls

- [row_to_memory](../../../../../../functions/src/db/repository/memory/row_to_memory.md)

# Called by

- [try_recall](../../../../../../functions/src/memory/MemoryStore/try_recall.md)
- [search](../../../../../../functions/src/memory/MemoryStore/search.md)
- [fts_search_finds_the_current_words_only](../../../../../../functions/tests/memory_repository_test/fts_search_finds_the_current_words_only.md)
- [fts_search_never_crosses_projects](../../../../../../functions/tests/memory_repository_test/fts_search_never_crosses_projects.md)
- [deleted_memory_is_gone_from_storage_and_search](../../../../../../functions/tests/memory_repository_test/deleted_memory_is_gone_from_storage_and_search.md)