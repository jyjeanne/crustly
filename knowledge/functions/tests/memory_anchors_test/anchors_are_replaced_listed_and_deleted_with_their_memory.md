---
type: Rust Function
title: anchors_are_replaced_listed_and_deleted_with_their_memory
resource: tests/memory_anchors_test.rs#L52-L88
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/set_anchors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_anchors_test/at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/anchors_of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/anchors_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn anchors_are_replaced_listed_and_deleted_with_their_memory()`

# Calls

- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [set_anchors](../../../functions/src/db/repository/memory/MemoryRepository/set_anchors.md)
- [at](../../../functions/tests/memory_anchors_test/at.md)
- [anchors_of](../../../functions/src/db/repository/memory/MemoryRepository/anchors_of.md)
- [forget](../../../functions/src/memory/MemoryStore/forget.md)
- [anchors_for](../../../functions/src/db/repository/memory/MemoryRepository/anchors_for.md)