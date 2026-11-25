---
type: Rust Module
title: memory_transfer_test
resource: tests/memory_transfer_test.rs#L1-L189
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
  - target: external/crustly-memory-project-id-for-root-identify-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-transfer-export-import-import-from-notes-exportscope
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

- [store](../../functions/tests/memory_transfer_test/store.md)
- [project_at](../../functions/tests/memory_transfer_test/project_at.md)
- [save](../../functions/tests/memory_transfer_test/save.md)
- [export_then_import_recreates_the_memories_in_a_new_database](../../functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database.md)
- [export_scope_limits_what_is_written](../../functions/tests/memory_transfer_test/export_scope_limits_what_is_written.md)
- [an_unknown_export_version_is_refused](../../functions/tests/memory_transfer_test/an_unknown_export_version_is_refused.md)
- [from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first](../../functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first.md)
- [from_notes_without_a_file_does_nothing](../../functions/tests/memory_transfer_test/from_notes_without_a_file_does_nothing.md)

# Imports

- `crustly::config::MemoryConfig`
- `crustly::db::Database`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `crustly::memory::project::{id_for_root, identify, ProjectIdentity}`
- `crustly::memory::transfer::{export, import, import_from_notes, ExportScope}`
- `crustly::memory::MemoryStore`
- `std::path::PathBuf`

# Member of

- [crustly](../../packages/crustly.md)