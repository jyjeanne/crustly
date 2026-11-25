---
type: Rust Function
title: a_second_put_replaces_the_first_vector
resource: tests/memory_vectors_test.rs#L94-L108
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

`async fn a_second_put_replaces_the_first_vector()`

# Calls

- [insert](../../../functions/tests/memory_vectors_test/insert.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [vectors_for](../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)