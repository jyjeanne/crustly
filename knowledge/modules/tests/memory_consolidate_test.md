---
type: Rust Module
title: memory_consolidate_test
resource: tests/memory_consolidate_test.rs#L1-L562
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-duration-utc
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
  - target: external/crustly-llm-provider-contentblock-llmrequest-llmresponse-provider-providerstream-result-as-providerresult-stopreason-tokenusage
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-embed-embederror-embedpurpose-embedder
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-memorystatus-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-id-for-root-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-semantic-embedruntime
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

- [now](../../functions/tests/memory_consolidate_test/now.md)
- [project](../../functions/tests/memory_consolidate_test/project.md)
- [setup](../../functions/tests/memory_consolidate_test/setup.md)
- [insert_at](../../functions/tests/memory_consolidate_test/insert_at.md)
- [an_expired_memory_is_archived_on_apply_and_only_reported_before](../../functions/tests/memory_consolidate_test/an_expired_memory_is_archived_on_apply_and_only_reported_before.md)
- [old_episodes_are_archived_and_recent_ones_kept](../../functions/tests/memory_consolidate_test/old_episodes_are_archived_and_recent_ones_kept.md)
- [a_year_unused_fact_is_listed_for_review_and_never_archived](../../functions/tests/memory_consolidate_test/a_year_unused_fact_is_listed_for_review_and_never_archived.md)
- [FixedEmbedder](../../classes/tests/memory_consolidate_test/FixedEmbedder.md)
- [model_id](../../functions/tests/memory_consolidate_test/FixedEmbedder/embedder/model_id.md)
- [embed](../../functions/tests/memory_consolidate_test/FixedEmbedder/embedder/embed.md)
- [near_duplicates_in_the_merge_band_are_merged_on_apply](../../functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply.md)
- [FixedAnswer](../../classes/tests/memory_consolidate_test/FixedAnswer.md)
- [new](../../functions/tests/memory_consolidate_test/FixedAnswer/new.md)
- [calls](../../functions/tests/memory_consolidate_test/FixedAnswer/calls.md)
- [complete](../../functions/tests/memory_consolidate_test/FixedAnswer/provider/complete.md)
- [stream](../../functions/tests/memory_consolidate_test/FixedAnswer/provider/stream.md)
- [name](../../functions/tests/memory_consolidate_test/FixedAnswer/provider/name.md)
- [default_model](../../functions/tests/memory_consolidate_test/FixedAnswer/provider/default_model.md)
- [supported_models](../../functions/tests/memory_consolidate_test/FixedAnswer/provider/supported_models.md)
- [context_window](../../functions/tests/memory_consolidate_test/FixedAnswer/provider/context_window.md)
- [calculate_cost](../../functions/tests/memory_consolidate_test/FixedAnswer/provider/calculate_cost.md)
- [three_episodes_become_one_lesson_with_its_sources](../../functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources.md)
- [extraction_on](../../functions/tests/memory_consolidate_test/extraction_on.md)
- [episodes_feed_one_lesson_only](../../functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only.md)
- [episodes_archived_by_the_same_run_do_not_feed_a_lesson](../../functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson.md)
- [lessons_need_extraction_on](../../functions/tests/memory_consolidate_test/lessons_need_extraction_on.md)
- [merge_band_vectors](../../functions/tests/memory_consolidate_test/merge_band_vectors.md)
- [the_merge_keeps_categories_apart](../../functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart.md)
- [the_merge_keeps_the_users_copy_over_a_newer_extracted_one](../../functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one.md)

# Imports

- `async_trait::async_trait`
- `chrono::{DateTime, Duration, Utc}`
- `crustly::config::MemoryConfig`
- `crustly::db::repository::memory::InsertMemory`
- `crustly::db::repository::MemoryRepository`
- `crustly::db::Database`
- `crustly::llm::provider::{
    ContentBlock, LLMRequest, LLMResponse, Provider, ProviderStream, Result as ProviderResult,
    StopReason, TokenUsage,
}`
- `crustly::memory::embed::{EmbedError, EmbedPurpose, Embedder}`
- `crustly::memory::model::{
    MemoryCategory, MemoryScope, MemorySource, MemoryStatus, NewMemory, SaveOutcome,
}`
- `crustly::memory::project::{id_for_root, ProjectIdentity}`
- `crustly::memory::semantic::EmbedRuntime`
- `crustly::memory::MemoryStore`
- `std::path::PathBuf`
- `std::sync::Arc`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)