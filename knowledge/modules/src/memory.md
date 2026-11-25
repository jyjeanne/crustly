---
type: Rust Module
title: memory
resource: src/memory/mod.rs#L1-L934
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-config-memoryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-repository-memory-insertmemory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-repository-memoryrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/model-content-hash-normalize-validate-memory-memorycategory-memoryscope-memorysource-memorystatus-newmemory-projectid-rejectreason-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/project-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/recall-ranked
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sqlx-sqlitepool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
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

- [RecallRequest](../../classes/src/memory/RecallRequest.md)
- [SearchScope](../../classes/src/memory/SearchScope.md)
- [MemoryStore](../../classes/src/memory/MemoryStore.md)
- [new](../../functions/src/memory/MemoryStore/new.md)
- [with_embedder](../../functions/src/memory/MemoryStore/with_embedder.md)
- [attach_embedder](../../functions/src/memory/MemoryStore/attach_embedder.md)
- [changed](../../functions/src/memory/MemoryStore/changed.md)
- [queue_embedding](../../functions/src/memory/MemoryStore/queue_embedding.md)
- [reindex](../../functions/src/memory/MemoryStore/reindex.md)
- [embedding_status](../../functions/src/memory/MemoryStore/embedding_status.md)
- [register_project](../../functions/src/memory/MemoryStore/register_project.md)
- [save](../../functions/src/memory/MemoryStore/save.md)
- [save_with](../../functions/src/memory/MemoryStore/save_with.md)
- [recall](../../functions/src/memory/MemoryStore/recall.md)
- [semantic_scores](../../functions/src/memory/MemoryStore/semantic_scores.md)
- [warn_query_once](../../functions/src/memory/MemoryStore/warn_query_once.md)
- [try_recall](../../functions/src/memory/MemoryStore/try_recall.md)
- [recent_work](../../functions/src/memory/MemoryStore/recent_work.md)
- [search](../../functions/src/memory/MemoryStore/search.md)
- [get](../../functions/src/memory/MemoryStore/get.md)
- [forget](../../functions/src/memory/MemoryStore/forget.md)
- [list](../../functions/src/memory/MemoryStore/list.md)
- [all](../../functions/src/memory/MemoryStore/all.md)
- [events](../../functions/src/memory/MemoryStore/events.md)
- [projects](../../functions/src/memory/MemoryStore/projects.md)
- [edit](../../functions/src/memory/MemoryStore/edit.md)
- [promote](../../functions/src/memory/MemoryStore/promote.md)
- [relink](../../functions/src/memory/MemoryStore/relink.md)
- [add_paths](../../functions/src/memory/MemoryStore/add_paths.md)
- [memories_for_path](../../functions/src/memory/MemoryStore/memories_for_path.md)
- [mark_shown](../../functions/src/memory/MemoryStore/mark_shown.md)
- [find](../../functions/src/memory/MemoryStore/find.md)
- [take_notices](../../functions/src/memory/MemoryStore/take_notices.md)
- [notify](../../functions/src/memory/MemoryStore/notify.md)
- [embedder_from_config](../../functions/src/memory/embedder_from_config.md)
- [actor_for](../../functions/src/memory/actor_for.md)

# Imports

- `crate::config::MemoryConfig`
- `crate::db::repository::memory::InsertMemory`
- `crate::db::repository::MemoryRepository`
- `anyhow::Result`
- `chrono::{DateTime, Utc}`
- `model::{
    content_hash, normalize, validate, Memory, MemoryCategory, MemoryScope, MemorySource,
    MemoryStatus, NewMemory, ProjectId, RejectReason, SaveOutcome,
}`
- `project::ProjectIdentity`
- `recall::Ranked`
- `sqlx::SqlitePool`
- `std::collections::HashMap`
- `std::path::Path`
- `std::sync::{Arc, Mutex}`
- `std::time::Duration`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)