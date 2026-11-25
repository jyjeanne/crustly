---
type: Rust Function
title: an_approved_memory_is_embedded
resource: tests/memory_semantic_test.rs#L494-L526
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/tests/memory_semantic_test/FakeEmbedder/working
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/fast_runtime
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/extraction/MemoryStore/approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/wait_for_vector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/topic_vector
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_approved_memory_is_embedded()`

# Calls

- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [with_embedder](../../../functions/src/memory/MemoryStore/with_embedder.md)
- [working](../../../functions/tests/memory_semantic_test/FakeEmbedder/working.md)
- [fast_runtime](../../../functions/tests/memory_semantic_test/fast_runtime.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [insert](../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [approve](../../../functions/src/memory/extraction/MemoryStore/approve.md)
- [wait_for_vector](../../../functions/tests/memory_semantic_test/wait_for_vector.md)
- [topic_vector](../../../functions/tests/memory_semantic_test/topic_vector.md)