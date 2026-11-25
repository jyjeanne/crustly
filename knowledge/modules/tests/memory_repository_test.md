---
type: Rust Module
title: memory_repository_test
resource: tests/memory_repository_test.rs#L1-L305
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/chrono-datetime-duration-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-repository-memory-insertmemory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-repository-memoryrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [fixed_now](../../functions/tests/memory_repository_test/fixed_now.md)
- [project](../../functions/tests/memory_repository_test/project.md)
- [new_memory](../../functions/tests/memory_repository_test/new_memory.md)
- [repo_with_projects](../../functions/tests/memory_repository_test/repo_with_projects.md)
- [project_scope](../../functions/tests/memory_repository_test/project_scope.md)
- [saved_memory_reads_back_unchanged](../../functions/tests/memory_repository_test/saved_memory_reads_back_unchanged.md)
- [find_by_hash_matches_only_the_same_scope](../../functions/tests/memory_repository_test/find_by_hash_matches_only_the_same_scope.md)
- [reinforce_raises_importance_up_to_five](../../functions/tests/memory_repository_test/reinforce_raises_importance_up_to_five.md)
- [candidates_hide_other_projects_and_inactive_memories](../../functions/tests/memory_repository_test/candidates_hide_other_projects_and_inactive_memories.md)
- [fts_search_finds_the_current_words_only](../../functions/tests/memory_repository_test/fts_search_finds_the_current_words_only.md)
- [fts_search_never_crosses_projects](../../functions/tests/memory_repository_test/fts_search_never_crosses_projects.md)
- [deleted_memory_is_gone_from_storage_and_search](../../functions/tests/memory_repository_test/deleted_memory_is_gone_from_storage_and_search.md)
- [supersede_keeps_history_and_hides_the_old_memory](../../functions/tests/memory_repository_test/supersede_keeps_history_and_hides_the_old_memory.md)
- [promote_makes_a_project_memory_global](../../functions/tests/memory_repository_test/promote_makes_a_project_memory_global.md)
- [relink_moves_memories_to_the_new_project](../../functions/tests/memory_repository_test/relink_moves_memories_to_the_new_project.md)

# Imports

- `chrono::{DateTime, Duration, Utc}`
- `crustly::db::repository::memory::InsertMemory`
- `crustly::db::repository::MemoryRepository`
- `crustly::db::Database`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, MemoryStatus}`
- `crustly::memory::project::ProjectIdentity`
- `std::path::PathBuf`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)