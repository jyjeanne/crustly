---
type: Rust Module
title: memory_paths_test
resource: tests/memory_paths_test.rs#L1-L285
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crustly-config-memoryconfig
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
  - target: external/crustly-memory-memorystore-recallrequest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
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

- [store_and_project](../../functions/tests/memory_paths_test/store_and_project.md)
- [save_in_project](../../functions/tests/memory_paths_test/save_in_project.md)
- [paths_must_be_relative_and_few](../../functions/tests/memory_paths_test/paths_must_be_relative_and_few.md)
- [a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget](../../functions/tests/memory_paths_test/a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget.md)
- [a_path_memory_is_shown_once_per_session](../../functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session.md)
- [store_and_real_project](../../functions/tests/memory_paths_test/store_and_real_project.md)
- [tool_paths_match_when_absolute_or_relative_to_a_subdirectory](../../functions/tests/memory_paths_test/tool_paths_match_when_absolute_or_relative_to_a_subdirectory.md)
- [an_absolute_path_in_the_message_brings_its_memory_in](../../functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in.md)

# Imports

- `crustly::config::MemoryConfig`
- `crustly::db::Database`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `crustly::memory::project::{id_for_root, ProjectIdentity}`
- `crustly::memory::{MemoryStore, RecallRequest}`
- `std::path::{Path, PathBuf}`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)