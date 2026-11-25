---
type: Rust Module
title: memory_store_test
resource: tests/memory_store_test.rs#L1-L371
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crustly-config-memoryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-repository-memoryrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-llm-agent-project-memory-append-progress-progressentry
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-newmemory-rejectreason-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-projectidentity
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

- [store](../../functions/tests/memory_store_test/store.md)
- [project](../../functions/tests/memory_store_test/project.md)
- [save_request](../../functions/tests/memory_store_test/save_request.md)
- [request](../../functions/tests/memory_store_test/request.md)
- [saving_twice_gives_one_memory](../../functions/tests/memory_store_test/saving_twice_gives_one_memory.md)
- [user_repeats_raise_importance_and_model_repeats_do_not](../../functions/tests/memory_store_test/user_repeats_raise_importance_and_model_repeats_do_not.md)
- [a_secret_is_rejected_and_nothing_is_stored](../../functions/tests/memory_store_test/a_secret_is_rejected_and_nothing_is_stored.md)
- [invalid_text_is_rejected_with_a_reason](../../functions/tests/memory_store_test/invalid_text_is_rejected_with_a_reason.md)
- [a_model_cannot_replace_a_memory_the_user_saved](../../functions/tests/memory_store_test/a_model_cannot_replace_a_memory_the_user_saved.md)
- [replacement_must_stay_in_the_same_scope](../../functions/tests/memory_store_test/replacement_must_stay_in_the_same_scope.md)
- [saves_are_announced_once](../../functions/tests/memory_store_test/saves_are_announced_once.md)
- [recall_includes_own_project_and_global_only](../../functions/tests/memory_store_test/recall_includes_own_project_and_global_only.md)
- [recall_is_none_when_nothing_is_stored](../../functions/tests/memory_store_test/recall_is_none_when_nothing_is_stored.md)
- [recent_work_is_added_only_on_the_first_request](../../functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request.md)
- [recall_records_what_it_sent](../../functions/tests/memory_store_test/recall_records_what_it_sent.md)

# Imports

- `crustly::config::MemoryConfig`
- `crustly::db::repository::MemoryRepository`
- `crustly::db::Database`
- `crustly::llm::agent::project_memory::{append_progress, ProgressEntry}`
- `crustly::memory::model::{
    MemoryCategory, MemoryScope, MemorySource, NewMemory, RejectReason, SaveOutcome,
}`
- `crustly::memory::project::ProjectIdentity`
- `crustly::memory::{MemoryStore, RecallRequest}`
- `std::path::{Path, PathBuf}`

# Member of

- [crustly](../../packages/crustly.md)