---
type: Rust Function
title: embed_and_store
resource: src/memory/semantic.rs#L96-L125
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/semantic/embed_with_retries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/put_embedding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/archive_near_duplicates
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/semantic/backfill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/spawn_worker
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn embed_and_store( repo: &MemoryRepository, embedder: &dyn Embedder, memories: &[Memory], retry_delays: &[Duration], generation: &AtomicU64, now: chrono::DateTime<chrono::Utc>, ) -> Result<usize, EmbedError>`

# Calls

- [embed_with_retries](../../../../functions/src/memory/semantic/embed_with_retries.md)
- [put_embedding](../../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [archive_near_duplicates](../../../../functions/src/memory/semantic/archive_near_duplicates.md)

# Called by

- [backfill](../../../../functions/src/memory/semantic/backfill.md)
- [spawn_worker](../../../../functions/src/memory/semantic/spawn_worker.md)