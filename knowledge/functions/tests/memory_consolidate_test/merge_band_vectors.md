---
type: Rust Function
title: merge_band_vectors
resource: tests/memory_consolidate_test.rs#L474-L487
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/put_embedding
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn merge_band_vectors(db: &Database, a: Uuid, b: Uuid)`

# Calls

- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)

# Called by

- [the_merge_keeps_categories_apart](../../../functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart.md)
- [the_merge_keeps_the_users_copy_over_a_newer_extracted_one](../../../functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one.md)