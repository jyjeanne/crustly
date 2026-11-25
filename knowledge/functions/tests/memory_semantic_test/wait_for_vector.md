---
type: Rust Function
title: wait_for_vector
resource: tests/memory_semantic_test.rs#L445-L466
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/vectors_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn wait_for_vector( db: &Database, project: &ProjectIdentity, id: uuid::Uuid, expected: &[f32], )`

# Calls

- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [vectors_for](../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)

# Called by

- [an_edited_memory_is_embedded_again_from_its_new_text](../../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)
- [an_approved_memory_is_embedded](../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)