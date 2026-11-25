---
type: Rust Module
title: consolidate
resource: src/memory/consolidate.rs#L1-L339
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-model-memory-memorycategory-memoryscope-memorysource-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-project-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-semantic-near-duplicate-to-archive
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-vector-similarity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-memorystore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-contentblock-llmrequest-message
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-duration-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ConsolidationReport](../../../classes/src/memory/consolidate/ConsolidationReport.md)
- [pair_similarity](../../../functions/src/memory/consolidate/pair_similarity.md)
- [is_stale](../../../functions/src/memory/consolidate/is_stale.md)
- [is_expired_episode](../../../functions/src/memory/consolidate/is_expired_episode.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [embedding_model](../../../functions/src/memory/consolidate/MemoryStore/embedding_model.md)
- [lesson_from_episodes](../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)
- [parse_lesson](../../../functions/src/memory/consolidate/parse_lesson.md)
- [memory](../../../functions/src/memory/consolidate/memory.md)
- [a_year_unused_fact_is_stale_but_a_preference_never_is](../../../functions/src/memory/consolidate/a_year_unused_fact_is_stale_but_a_preference_never_is.md)
- [episodes_expire_after_the_retention_period](../../../functions/src/memory/consolidate/episodes_expire_after_the_retention_period.md)
- [a_lesson_is_read_from_the_first_json_object](../../../functions/src/memory/consolidate/a_lesson_is_read_from_the_first_json_object.md)

# Imports

- `super::model::{Memory, MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `super::project::ProjectIdentity`
- `super::semantic::near_duplicate_to_archive`
- `super::vector::similarity`
- `super::MemoryStore`
- `crate::llm::provider::{ContentBlock, LLMRequest, Message}`
- `anyhow::Result`
- `chrono::{DateTime, Duration, Utc}`
- `std::collections::HashSet`
- `uuid::Uuid`
- `super::*`
- `crate::memory::model::MemoryStatus`

# Member of

- [crustly](../../../packages/crustly.md)