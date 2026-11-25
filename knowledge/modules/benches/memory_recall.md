---
type: Rust Module
title: memory_recall
resource: benches/memory_recall.rs#L1-L97
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/criterion-criterion-group-criterion-main-benchmarkid-criterion
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-config-memoryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-vector-normalize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-vector-index-vectorindex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-hint-black-box
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

- [vector](../../functions/benches/memory_recall/vector.md)
- [index_of](../../functions/benches/memory_recall/index_of.md)
- [bench_vector_search](../../functions/benches/memory_recall/bench_vector_search.md)
- [bench_keyword_recall](../../functions/benches/memory_recall/bench_keyword_recall.md)

# Imports

- `criterion::{criterion_group, criterion_main, BenchmarkId, Criterion}`
- `crustly::config::MemoryConfig`
- `crustly::db::Database`
- `crustly::memory::vector::normalize`
- `crustly::memory::vector_index::VectorIndex`
- `std::hint::black_box`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)