---
type: Rust Module
title: memory_review_fixes_test
resource: tests/memory_review_fixes_test.rs#L1-L166
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
  - target: external/crustly-db-repository-memory-insertmemory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-repository-memoryrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-llm-tools-forget-memory-forgetmemorytool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-llm-tools-tool-toolexecutioncontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-memorystore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
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

- [now](../../functions/tests/memory_review_fixes_test/now.md)
- [project](../../functions/tests/memory_review_fixes_test/project.md)
- [repo](../../functions/tests/memory_review_fixes_test/repo.md)
- [memory](../../functions/tests/memory_review_fixes_test/memory.md)
- [relinking_a_project_to_itself_is_refused_and_deletes_nothing](../../functions/tests/memory_review_fixes_test/relinking_a_project_to_itself_is_refused_and_deletes_nothing.md)
- [relinking_moves_a_duplicate_as_archived_and_the_rest_as_active](../../functions/tests/memory_review_fixes_test/relinking_moves_a_duplicate_as_archived_and_the_rest_as_active.md)
- [the_database_listing_covers_every_project](../../functions/tests/memory_review_fixes_test/the_database_listing_covers_every_project.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)

# Imports

- `chrono::{DateTime, Utc}`
- `crustly::config::MemoryConfig`
- `crustly::db::repository::memory::InsertMemory`
- `crustly::db::repository::MemoryRepository`
- `crustly::db::Database`
- `crustly::llm::tools::forget_memory::ForgetMemoryTool`
- `crustly::llm::tools::{Tool, ToolExecutionContext}`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, MemoryStatus}`
- `crustly::memory::project::ProjectIdentity`
- `crustly::memory::MemoryStore`
- `std::path::PathBuf`
- `std::sync::Arc`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)