---
type: Rust Module
title: memory_anchors_test
resource: tests/memory_anchors_test.rs#L1-L109
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-config-memoryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-repository-memoryrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-id-for-root-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-memorystore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [setup](../../functions/tests/memory_anchors_test/setup.md)
- [save](../../functions/tests/memory_anchors_test/save.md)
- [at](../../functions/tests/memory_anchors_test/at.md)
- [anchors_are_replaced_listed_and_deleted_with_their_memory](../../functions/tests/memory_anchors_test/anchors_are_replaced_listed_and_deleted_with_their_memory.md)
- [anchors_of_archived_memories_are_not_listed_for_the_project](../../functions/tests/memory_anchors_test/anchors_of_archived_memories_are_not_listed_for_the_project.md)

# Imports

- `chrono::{DateTime, Utc}`
- `crustly::config::MemoryConfig`
- `crustly::db::repository::MemoryRepository`
- `crustly::db::Database`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `crustly::memory::project::{id_for_root, ProjectIdentity}`
- `crustly::memory::MemoryStore`
- `std::path::PathBuf`

# Member of

- [crustly](../../packages/crustly.md)