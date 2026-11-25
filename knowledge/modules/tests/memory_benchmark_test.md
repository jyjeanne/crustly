---
type: Rust Module
title: memory_benchmark_test
resource: tests/memory_benchmark_test.rs#L1-L310
generated:
  by: okf-rs/0.8.0
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
  - target: external/crustly-llm-agent-context-token-count
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-embed-embederror-embedpurpose-embedsettings-embedder-ollamaembedder-prefixes
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-newmemory-saveoutcome
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
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Fixture](../../classes/tests/memory_benchmark_test/Fixture.md)
- [FixtureMemory](../../classes/tests/memory_benchmark_test/FixtureMemory.md)
- [FixtureQuery](../../classes/tests/memory_benchmark_test/FixtureQuery.md)
- [load_fixture](../../functions/tests/memory_benchmark_test/load_fixture.md)
- [Metrics](../../classes/tests/memory_benchmark_test/Metrics.md)
- [TopicEmbedder](../../classes/tests/memory_benchmark_test/TopicEmbedder.md)
- [model_id](../../functions/tests/memory_benchmark_test/TopicEmbedder/embedder/model_id.md)
- [embed](../../functions/tests/memory_benchmark_test/TopicEmbedder/embedder/embed.md)
- [run](../../functions/tests/memory_benchmark_test/run.md)
- [report](../../functions/tests/memory_benchmark_test/report.md)
- [the_harness_runs_and_meets_the_ci_floor](../../functions/tests/memory_benchmark_test/the_harness_runs_and_meets_the_ci_floor.md)
- [real_embedder_benchmark](../../functions/tests/memory_benchmark_test/real_embedder_benchmark.md)

# Imports

- `async_trait::async_trait`
- `crustly::config::MemoryConfig`
- `crustly::db::Database`
- `crustly::llm::agent::context::token_count`
- `crustly::memory::embed::{
    EmbedError, EmbedPurpose, EmbedSettings, Embedder, OllamaEmbedder, Prefixes,
}`
- `crustly::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `crustly::memory::project::{id_for_root, ProjectIdentity}`
- `crustly::memory::semantic::EmbedRuntime`
- `crustly::memory::vector::normalize`
- `crustly::memory::{MemoryStore, RecallRequest}`
- `serde::Deserialize`
- `std::collections::{HashMap, HashSet}`
- `std::path::{Path, PathBuf}`
- `std::sync::Arc`
- `std::time::Duration`

# Member of

- [crustly](../../packages/crustly.md)