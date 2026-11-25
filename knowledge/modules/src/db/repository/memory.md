---
type: Rust Module
title: memory
resource: src/db/repository/memory.rs#L1-L853
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crate-db-retry-retry-db-sqlx-dbretryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memory-memorycategory-memoryscope-memorysource-memorystatus-projectid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-project-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sqlx-row-sqlitepool
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

- [InsertMemory](../../../../classes/src/db/repository/memory/InsertMemory.md)
- [MemoryRepository](../../../../classes/src/db/repository/memory/MemoryRepository.md)
- [new](../../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [pool](../../../../functions/src/db/repository/memory/MemoryRepository/pool.md)
- [upsert_project](../../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)
- [insert](../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [find_by_hash](../../../../functions/src/db/repository/memory/MemoryRepository/find_by_hash.md)
- [archive](../../../../functions/src/db/repository/memory/MemoryRepository/archive.md)
- [extraction_state](../../../../functions/src/db/repository/memory/MemoryRepository/extraction_state.md)
- [set_extraction_state](../../../../functions/src/db/repository/memory/MemoryRepository/set_extraction_state.md)
- [turns_after](../../../../functions/src/db/repository/memory/MemoryRepository/turns_after.md)
- [approve](../../../../functions/src/db/repository/memory/MemoryRepository/approve.md)
- [set_pending_replaces](../../../../functions/src/db/repository/memory/MemoryRepository/set_pending_replaces.md)
- [pending_replaces](../../../../functions/src/db/repository/memory/MemoryRepository/pending_replaces.md)
- [pending_for](../../../../functions/src/db/repository/memory/MemoryRepository/pending_for.md)
- [episodes_for](../../../../functions/src/db/repository/memory/MemoryRepository/episodes_for.md)
- [add_paths](../../../../functions/src/db/repository/memory/MemoryRepository/add_paths.md)
- [paths_of](../../../../functions/src/db/repository/memory/MemoryRepository/paths_of.md)
- [path_globs_for](../../../../functions/src/db/repository/memory/MemoryRepository/path_globs_for.md)
- [folded_sources](../../../../functions/src/db/repository/memory/MemoryRepository/folded_sources.md)
- [add_sources](../../../../functions/src/db/repository/memory/MemoryRepository/add_sources.md)
- [touch](../../../../functions/src/db/repository/memory/MemoryRepository/touch.md)
- [reinforce](../../../../functions/src/db/repository/memory/MemoryRepository/reinforce.md)
- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [find_by_prefix](../../../../functions/src/db/repository/memory/MemoryRepository/find_by_prefix.md)
- [candidates](../../../../functions/src/db/repository/memory/MemoryRepository/candidates.md)
- [list_for_project](../../../../functions/src/db/repository/memory/MemoryRepository/list_for_project.md)
- [list_all](../../../../functions/src/db/repository/memory/MemoryRepository/list_all.md)
- [events](../../../../functions/src/db/repository/memory/MemoryRepository/events.md)
- [projects](../../../../functions/src/db/repository/memory/MemoryRepository/projects.md)
- [record_recall](../../../../functions/src/db/repository/memory/MemoryRepository/record_recall.md)
- [fts_search](../../../../functions/src/db/repository/memory/MemoryRepository/fts_search.md)
- [update_content](../../../../functions/src/db/repository/memory/MemoryRepository/update_content.md)
- [supersede](../../../../functions/src/db/repository/memory/MemoryRepository/supersede.md)
- [promote](../../../../functions/src/db/repository/memory/MemoryRepository/promote.md)
- [delete](../../../../functions/src/db/repository/memory/MemoryRepository/delete.md)
- [relink_project](../../../../functions/src/db/repository/memory/MemoryRepository/relink_project.md)
- [put_embedding](../../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [vectors_for](../../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)
- [missing_embeddings](../../../../functions/src/db/repository/memory/MemoryRepository/missing_embeddings.md)
- [prune_embeddings_except](../../../../functions/src/db/repository/memory/MemoryRepository/prune_embeddings_except.md)
- [embedding_counts](../../../../functions/src/db/repository/memory/MemoryRepository/embedding_counts.md)
- [scope_columns](../../../../functions/src/db/repository/memory/scope_columns.md)
- [record_event](../../../../functions/src/db/repository/memory/record_event.md)
- [timestamp](../../../../functions/src/db/repository/memory/timestamp.md)
- [row_to_memory](../../../../functions/src/db/repository/memory/row_to_memory.md)

# Imports

- `crate::db::retry::{retry_db_sqlx, DbRetryConfig}`
- `crate::memory::model::{
    Memory, MemoryCategory, MemoryScope, MemorySource, MemoryStatus, ProjectId,
}`
- `crate::memory::project::ProjectIdentity`
- `anyhow::{Context, Result}`
- `chrono::{DateTime, Utc}`
- `sqlx::{Row, SqlitePool}`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)