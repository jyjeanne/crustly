---
type: Rust Function
title: reindex_embeds_memories_saved_before_the_embedder_was_configured
resource: tests/memory_semantic_test.rs#L269-L295
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
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/with_embedder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/FakeEmbedder/working
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/fast_runtime
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/reindex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/wait_until_indexed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/embedding_status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn reindex_embeds_memories_saved_before_the_embedder_was_configured()`

# Calls

- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [with_embedder](../../../functions/src/memory/MemoryStore/with_embedder.md)
- [working](../../../functions/tests/memory_semantic_test/FakeEmbedder/working.md)
- [fast_runtime](../../../functions/tests/memory_semantic_test/fast_runtime.md)
- [reindex](../../../functions/src/memory/MemoryStore/reindex.md)
- [wait_until_indexed](../../../functions/tests/memory_semantic_test/wait_until_indexed.md)
- [embedding_status](../../../functions/src/memory/MemoryStore/embedding_status.md)