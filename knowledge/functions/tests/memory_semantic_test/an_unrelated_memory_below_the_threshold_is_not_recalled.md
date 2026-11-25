---
type: Rust Function
title: an_unrelated_memory_below_the_threshold_is_not_recalled
resource: tests/memory_semantic_test.rs#L177-L214
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
  - target: functions/tests/memory_semantic_test/wait_until_indexed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/recall_in_window
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_unrelated_memory_below_the_threshold_is_not_recalled()`

# Calls

- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [with_embedder](../../../functions/src/memory/MemoryStore/with_embedder.md)
- [working](../../../functions/tests/memory_semantic_test/FakeEmbedder/working.md)
- [fast_runtime](../../../functions/tests/memory_semantic_test/fast_runtime.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [wait_until_indexed](../../../functions/tests/memory_semantic_test/wait_until_indexed.md)
- [recall_in_window](../../../functions/tests/memory_semantic_test/recall_in_window.md)