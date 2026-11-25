---
type: Rust Function
title: prune_keeps_only_the_current_model_and_counts_match
resource: tests/memory_vectors_test.rs#L178-L187
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
  - target: functions/src/db/repository/memory/MemoryRepository/embedding_counts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/prune_embeddings_except
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn prune_keeps_only_the_current_model_and_counts_match()`

# Calls

- [insert](../../../functions/tests/memory_vectors_test/insert.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [embedding_counts](../../../functions/src/db/repository/memory/MemoryRepository/embedding_counts.md)
- [prune_embeddings_except](../../../functions/src/db/repository/memory/MemoryRepository/prune_embeddings_except.md)