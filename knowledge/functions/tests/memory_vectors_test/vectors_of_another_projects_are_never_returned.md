---
type: Rust Function
title: vectors_of_another_projects_are_never_returned
resource: tests/memory_vectors_test.rs#L81-L91
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/upsert_project
    resolved_by: tree-sitter
    confidence: exact
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

`async fn vectors_of_another_projects_are_never_returned()`

# Calls

- [upsert_project](../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)
- [insert](../../../functions/tests/memory_vectors_test/insert.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [vectors_for](../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)