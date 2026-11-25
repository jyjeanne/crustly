---
type: Rust Module
title: memory_minor_fixes_test
resource: tests/memory_minor_fixes_test.rs#L1-L148
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-config-memoryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-llm-agent-context-token-count
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memory-memorycategory-memoryscope-memorysource-memorystatus-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-identify
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-recall-select-ranked
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-transfer-remove-bullets
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-memorystore
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

- [store](../../functions/tests/memory_minor_fixes_test/store.md)
- [global](../../functions/tests/memory_minor_fixes_test/global.md)
- [a_duplicate_still_supersedes_the_memory_it_replaces](../../functions/tests/memory_minor_fixes_test/a_duplicate_still_supersedes_the_memory_it_replaces.md)
- [removing_a_bullet_keeps_the_crlf_endings_of_the_other_lines](../../functions/tests/memory_minor_fixes_test/removing_a_bullet_keeps_the_crlf_endings_of_the_other_lines.md)
- [a_submodule_is_its_own_project_not_its_superprojects](../../functions/tests/memory_minor_fixes_test/a_submodule_is_its_own_project_not_its_superprojects.md)
- [memory](../../functions/tests/memory_minor_fixes_test/memory.md)
- [the_budget_counts_each_lines_overhead_not_just_its_text](../../functions/tests/memory_minor_fixes_test/the_budget_counts_each_lines_overhead_not_just_its_text.md)

# Imports

- `chrono::{DateTime, Utc}`
- `crustly::config::MemoryConfig`
- `crustly::db::Database`
- `crustly::llm::agent::context::token_count`
- `crustly::memory::model::{
    Memory, MemoryCategory, MemoryScope, MemorySource, MemoryStatus, NewMemory, SaveOutcome,
}`
- `crustly::memory::project::identify`
- `crustly::memory::recall::{select, Ranked}`
- `crustly::memory::transfer::remove_bullets`
- `crustly::memory::MemoryStore`
- `std::path::PathBuf`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)