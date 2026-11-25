---
type: Rust Module
title: memory_semantic_test
resource: tests/memory_semantic_test.rs#L1-L567
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-config-memoryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-embed-embederror-embedpurpose-embedder
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-newmemory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-id-for-root-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-semantic-embedruntime
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-vector-normalize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-memorystore-recallrequest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-atomic-atomicusize-ordering
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-config-memoryembeddingsconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-embedder-from-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-config-memoryconfig-memoryembeddingsconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-repository-memory-insertmemory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [topic_vector](../../functions/tests/memory_semantic_test/topic_vector.md)
- [FakeEmbedder](../../classes/tests/memory_semantic_test/FakeEmbedder.md)
- [working](../../functions/tests/memory_semantic_test/FakeEmbedder/working.md)
- [model_id](../../functions/tests/memory_semantic_test/FakeEmbedder/embedder/model_id.md)
- [embed](../../functions/tests/memory_semantic_test/FakeEmbedder/embedder/embed.md)
- [fast_runtime](../../functions/tests/memory_semantic_test/fast_runtime.md)
- [project](../../functions/tests/memory_semantic_test/project.md)
- [database](../../functions/tests/memory_semantic_test/database.md)
- [save](../../functions/tests/memory_semantic_test/save.md)
- [wait_until_indexed](../../functions/tests/memory_semantic_test/wait_until_indexed.md)
- [recall](../../functions/tests/memory_semantic_test/recall.md)
- [recall_in_window](../../functions/tests/memory_semantic_test/recall_in_window.md)
- [a_memory_is_recalled_by_meaning_without_a_shared_word](../../functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word.md)
- [an_unrelated_memory_below_the_threshold_is_not_recalled](../../functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled.md)
- [when_the_embedder_is_down_keyword_recall_still_works](../../functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works.md)
- [a_failed_embedding_is_retried_and_then_stored](../../functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored.md)
- [reindex_embeds_memories_saved_before_the_embedder_was_configured](../../functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured.md)
- [two_memories_saying_the_same_thing_are_recalled_once](../../functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once.md)
- [the_embeddings_settings_build_the_right_provider_or_none](../../functions/tests/memory_semantic_test/the_embeddings_settings_build_the_right_provider_or_none.md)
- [bad_embeddings_settings_fall_back_to_defaults_at_load](../../functions/tests/memory_semantic_test/bad_embeddings_settings_fall_back_to_defaults_at_load.md)
- [SameVectorEmbedder](../../classes/tests/memory_semantic_test/SameVectorEmbedder.md)
- [model_id](../../functions/tests/memory_semantic_test/SameVectorEmbedder/embedder/model_id.md)
- [embed](../../functions/tests/memory_semantic_test/SameVectorEmbedder/embedder/embed.md)
- [one_of_two_near_duplicates_is_archived_not_deleted](../../functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted.md)
- [wait_for_vector](../../functions/tests/memory_semantic_test/wait_for_vector.md)
- [an_edited_memory_is_embedded_again_from_its_new_text](../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)
- [an_approved_memory_is_embedded](../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)
- [a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget](../../functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget.md)

# Imports

- `async_trait::async_trait`
- `crustly::config::MemoryConfig`
- `crustly::db::Database`
- `crustly::memory::embed::{EmbedError, EmbedPurpose, Embedder}`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory}`
- `crustly::memory::project::{id_for_root, ProjectIdentity}`
- `crustly::memory::semantic::EmbedRuntime`
- `crustly::memory::vector::normalize`
- `crustly::memory::{MemoryStore, RecallRequest}`
- `std::path::Path`
- `std::sync::atomic::{AtomicUsize, Ordering}`
- `std::sync::Arc`
- `std::time::Duration`
- `crustly::config::MemoryEmbeddingsConfig`
- `crustly::memory::embedder_from_config`
- `crustly::config::{MemoryConfig, MemoryEmbeddingsConfig}`
- `crustly::db::repository::memory::InsertMemory`
- `crustly::memory::model::MemoryStatus`

# Member of

- [crustly](../../packages/crustly.md)