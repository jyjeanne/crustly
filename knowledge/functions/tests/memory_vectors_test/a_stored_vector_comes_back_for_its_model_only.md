---
type: Rust Function
title: a_stored_vector_comes_back_for_its_model_only
resource: tests/memory_vectors_test.rs#L58-L78
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_vectors_test/insert
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/put_embedding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/vectors_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_stored_vector_comes_back_for_its_model_only()`

# Calls

- [insert](../../../functions/tests/memory_vectors_test/insert.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [vectors_for](../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)