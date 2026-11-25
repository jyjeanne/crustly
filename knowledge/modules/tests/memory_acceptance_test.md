---
type: Rust Module
title: memory_acceptance_test
resource: tests/memory_acceptance_test.rs#L1-L291
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
  - target: external/crustly-memory-project-identify-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-memorystore-recallrequest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [open](../../functions/tests/memory_acceptance_test/open.md)
- [repo](../../functions/tests/memory_acceptance_test/repo.md)
- [save](../../functions/tests/memory_acceptance_test/save.md)
- [recall](../../functions/tests/memory_acceptance_test/recall.md)
- [a_preference_survives_a_restart_and_is_recalled_in_another_repository](../../functions/tests/memory_acceptance_test/a_preference_survives_a_restart_and_is_recalled_in_another_repository.md)
- [a_project_fact_is_never_recalled_in_another_repository](../../functions/tests/memory_acceptance_test/a_project_fact_is_never_recalled_in_another_repository.md)
- [an_updated_decision_supersedes_the_old_one_and_keeps_its_history](../../functions/tests/memory_acceptance_test/an_updated_decision_supersedes_the_old_one_and_keeps_its_history.md)
- [a_secret_never_reaches_the_database_file_or_its_wal](../../functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal.md)
- [contains](../../functions/tests/memory_acceptance_test/contains.md)
- [identify_gives_the_same_project_for_the_same_directory](../../functions/tests/memory_acceptance_test/identify_gives_the_same_project_for_the_same_directory.md)

# Imports

- `crustly::config::MemoryConfig`
- `crustly::db::Database`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `crustly::memory::project::{identify, ProjectIdentity}`
- `crustly::memory::{MemoryStore, RecallRequest}`
- `std::path::{Path, PathBuf}`

# Member of

- [crustly](../../packages/crustly.md)