---
type: Rust Function
title: a_vector_whose_size_does_not_match_its_record_is_skipped
resource: tests/memory_vectors_test.rs#L111-L135
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
  - target: functions/src/memory/vector/encode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/vectors_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_vector_whose_size_does_not_match_its_record_is_skipped()`

# Calls

- [insert](../../../functions/tests/memory_vectors_test/insert.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [encode](../../../functions/src/memory/vector/encode.md)
- [vectors_for](../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)