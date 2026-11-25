---
type: Rust Module
title: memory_vectors_test
resource: tests/memory_vectors_test.rs#L1-L187
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/chrono-datetime-utc
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

- [now](../../functions/tests/memory_vectors_test/now.md)
- [project](../../functions/tests/memory_vectors_test/project.md)
- [setup](../../functions/tests/memory_vectors_test/setup.md)
- [insert](../../functions/tests/memory_vectors_test/insert.md)
- [a_stored_vector_comes_back_for_its_model_only](../../functions/tests/memory_vectors_test/a_stored_vector_comes_back_for_its_model_only.md)
- [vectors_of_another_projects_are_never_returned](../../functions/tests/memory_vectors_test/vectors_of_another_projects_are_never_returned.md)
- [a_second_put_replaces_the_first_vector](../../functions/tests/memory_vectors_test/a_second_put_replaces_the_first_vector.md)
- [a_vector_whose_size_does_not_match_its_record_is_skipped](../../functions/tests/memory_vectors_test/a_vector_whose_size_does_not_match_its_record_is_skipped.md)
- [forgetting_a_memory_removes_its_vector](../../functions/tests/memory_vectors_test/forgetting_a_memory_removes_its_vector.md)
- [missing_lists_active_memories_without_a_vector_for_the_model](../../functions/tests/memory_vectors_test/missing_lists_active_memories_without_a_vector_for_the_model.md)
- [prune_keeps_only_the_current_model_and_counts_match](../../functions/tests/memory_vectors_test/prune_keeps_only_the_current_model_and_counts_match.md)

# Imports

- `chrono::{DateTime, Utc}`
- `crustly::db::repository::memory::InsertMemory`
- `crustly::db::repository::MemoryRepository`
- `crustly::db::Database`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, MemoryStatus}`
- `crustly::memory::project::ProjectIdentity`
- `std::path::PathBuf`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)