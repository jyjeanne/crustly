---
type: Rust Function
title: near_duplicates_in_the_merge_band_are_merged_on_apply
resource: tests/memory_consolidate_test.rs#L191-L264
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/attach_embedder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/EmbedRuntime/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/put_embedding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn near_duplicates_in_the_merge_band_are_merged_on_apply()`

# Calls

- [attach_embedder](../../../functions/src/memory/MemoryStore/attach_embedder.md)
- [default](../../../functions/src/memory/semantic/EmbedRuntime/default/default.md)
- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [put_embedding](../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)