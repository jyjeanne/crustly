---
type: Rust Function
title: archive_near_duplicates
resource: src/memory/semantic.rs#L305-L353
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/vectors_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector/similarity
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/semantic/near_duplicate_to_archive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/archive
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/memory/semantic/embed_and_store
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn archive_near_duplicates( repo: &MemoryRepository, model: &str, memories: &[Memory], now: chrono::DateTime<chrono::Utc>, ) -> anyhow::Result<usize>`

# Calls

- [vectors_for](../../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)
- [find](../../../../functions/src/memory/MemoryStore/find.md)
- [similarity](../../../../functions/src/memory/vector/similarity.md)
- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [near_duplicate_to_archive](../../../../functions/src/memory/semantic/near_duplicate_to_archive.md)
- [archive](../../../../functions/src/db/repository/memory/MemoryRepository/archive.md)

# Called by

- [embed_and_store](../../../../functions/src/memory/semantic/embed_and_store.md)