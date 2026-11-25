---
type: Rust Function
title: missing_lists_active_memories_without_a_vector_for_the_model
resource: tests/memory_vectors_test.rs#L154-L175
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_vectors_test/insert
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/put_embedding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/missing_embeddings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn missing_lists_active_memories_without_a_vector_for_the_model()`

# Calls

- [insert](../../../functions/tests/memory_vectors_test/insert.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [missing_embeddings](../../../functions/src/db/repository/memory/MemoryRepository/missing_embeddings.md)