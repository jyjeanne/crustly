---
type: Rust Method
title: missing_embeddings
resource: src/db/repository/memory.rs#L826-L838
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/semantic/backfill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/missing_lists_active_memories_without_a_vector_for_the_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn missing_embeddings(&self, model: &str, limit: i64) -> Result<Vec<Memory>>`

# Called by

- [backfill](../../../../../../functions/src/memory/semantic/backfill.md)
- [missing_lists_active_memories_without_a_vector_for_the_model](../../../../../../functions/tests/memory_vectors_test/missing_lists_active_memories_without_a_vector_for_the_model.md)