---
type: Rust Function
title: run
resource: tests/memory_benchmark_test.rs#L116-L247
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_benchmark_test/load_fixture
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/attach_embedder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/reindex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn run(embedder: Option<(Arc<dyn Embedder>, EmbedRuntime)>) -> Metrics`

# Calls

- [load_fixture](../../../functions/tests/memory_benchmark_test/load_fixture.md)
- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [attach_embedder](../../../functions/src/memory/MemoryStore/attach_embedder.md)
- [id_for_root](../../../functions/src/memory/project/id_for_root.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [save](../../../functions/src/memory/MemoryStore/save.md)
- [reindex](../../../functions/src/memory/MemoryStore/reindex.md)
- [recall](../../../functions/src/memory/MemoryStore/recall.md)
- [token_count](../../../functions/src/llm/agent/context/token_count.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)