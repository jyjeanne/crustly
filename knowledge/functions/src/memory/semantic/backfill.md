---
type: Rust Function
title: backfill
resource: src/memory/semantic.rs#L129-L154
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/missing_embeddings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/embed_and_store
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/reindex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/spawn_worker
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn backfill( repo: &MemoryRepository, embedder: &dyn Embedder, runtime: &EmbedRuntime, generation: &AtomicU64, ) -> Result<usize, EmbedError>`

# Calls

- [missing_embeddings](../../../../functions/src/db/repository/memory/MemoryRepository/missing_embeddings.md)
- [embed_and_store](../../../../functions/src/memory/semantic/embed_and_store.md)

# Called by

- [reindex](../../../../functions/src/memory/MemoryStore/reindex.md)
- [spawn_worker](../../../../functions/src/memory/semantic/spawn_worker.md)