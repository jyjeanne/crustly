---
type: Rust Function
title: forgetting_a_memory_removes_its_vector
resource: tests/memory_vectors_test.rs#L138-L151
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
---

# Signature

`async fn forgetting_a_memory_removes_its_vector()`

# Calls

- [insert](../../../functions/tests/memory_vectors_test/insert.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)