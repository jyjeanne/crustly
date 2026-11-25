---
type: Rust Module
title: semantic
resource: src/memory/semantic.rs#L1-L414
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-embed-embederror-embedpurpose-embedder
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-model-memory-projectid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-vector-index-vectorindex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-repository-memoryrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-atomic-atomicu64-ordering
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-mpsc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-model-memorycategory-memorysource
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-model-memoryscope-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-vector-similarity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memorycategory-memoryscope-memorysource-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-duration
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [EmbedRuntime](../../../classes/src/memory/semantic/EmbedRuntime.md)
- [default](../../../functions/src/memory/semantic/EmbedRuntime/default/default.md)
- [EmbedHandle](../../../classes/src/memory/semantic/EmbedHandle.md)
- [CachedIndex](../../../classes/src/memory/semantic/CachedIndex.md)
- [embed_with_retries](../../../functions/src/memory/semantic/embed_with_retries.md)
- [embed_and_store](../../../functions/src/memory/semantic/embed_and_store.md)
- [backfill](../../../functions/src/memory/semantic/backfill.md)
- [spawn_worker](../../../functions/src/memory/semantic/spawn_worker.md)
- [queue](../../../functions/src/memory/semantic/queue.md)
- [current_index](../../../functions/src/memory/semantic/current_index.md)
- [normalise](../../../functions/src/memory/semantic/normalise.md)
- [drop_near_duplicates](../../../functions/src/memory/semantic/drop_near_duplicates.md)
- [SemanticHits](../../../classes/src/memory/semantic/SemanticHits.md)
- [near_duplicate_to_archive](../../../functions/src/memory/semantic/near_duplicate_to_archive.md)
- [archive_near_duplicates](../../../functions/src/memory/semantic/archive_near_duplicates.md)
- [memory](../../../functions/src/memory/semantic/memory.md)
- [loser](../../../functions/src/memory/semantic/loser.md)
- [the_newer_near_duplicate_stays](../../../functions/src/memory/semantic/the_newer_near_duplicate_stays.md)
- [a_user_memory_beats_a_newer_one_the_user_did_not_save](../../../functions/src/memory/semantic/a_user_memory_beats_a_newer_one_the_user_did_not_save.md)
- [episodes_and_different_categories_are_both_kept](../../../functions/src/memory/semantic/episodes_and_different_categories_are_both_kept.md)

# Imports

- `super::embed::{EmbedError, EmbedPurpose, Embedder}`
- `super::model::{Memory, ProjectId}`
- `super::vector_index::VectorIndex`
- `crate::db::repository::MemoryRepository`
- `std::sync::atomic::{AtomicU64, Ordering}`
- `std::sync::Arc`
- `std::time::Duration`
- `tokio::sync::mpsc`
- `uuid::Uuid`
- `super::model::{MemoryCategory, MemorySource}`
- `super::model::{MemoryScope, MemoryStatus}`
- `super::vector::similarity`
- `super::*`
- `crate::memory::model::{MemoryCategory, MemoryScope, MemorySource, MemoryStatus}`
- `chrono::{DateTime, Duration}`

# Member of

- [crustly](../../../packages/crustly.md)