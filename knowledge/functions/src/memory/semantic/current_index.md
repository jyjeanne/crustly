---
type: Rust Function
title: current_index
resource: src/memory/semantic.rs#L209-L243
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/vectors_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/from_entries
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/semantic_scores
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn current_index( cache: &std::sync::Mutex<Option<CachedIndex>>, generation: &AtomicU64, repo: &MemoryRepository, project: &ProjectId, model: &str, now: chrono::DateTime<chrono::Utc>, ) -> anyhow::Result<Arc<VectorIndex>>`

# Calls

- [model](../../../../functions/src/memory/vector_index/VectorIndex/model.md)
- [vectors_for](../../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)
- [from_entries](../../../../functions/src/memory/vector_index/VectorIndex/from_entries.md)

# Called by

- [semantic_scores](../../../../functions/src/memory/MemoryStore/semantic_scores.md)