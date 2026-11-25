---
type: Rust Function
title: one_of_two_near_duplicates_is_archived_not_deleted
resource: tests/memory_semantic_test.rs#L400-L442
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/with_embedder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/fast_runtime
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn one_of_two_near_duplicates_is_archived_not_deleted()`

# Calls

- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [with_embedder](../../../functions/src/memory/MemoryStore/with_embedder.md)
- [fast_runtime](../../../functions/tests/memory_semantic_test/fast_runtime.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [list](../../../functions/src/memory/MemoryStore/list.md)