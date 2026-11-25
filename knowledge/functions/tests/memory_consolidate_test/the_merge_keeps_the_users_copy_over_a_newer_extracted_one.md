---
type: Rust Function
title: the_merge_keeps_the_users_copy_over_a_newer_extracted_one
resource: tests/memory_consolidate_test.rs#L522-L562
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
  - target: functions/tests/memory_consolidate_test/insert_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/merge_band_vectors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn the_merge_keeps_the_users_copy_over_a_newer_extracted_one()`

# Calls

- [attach_embedder](../../../functions/src/memory/MemoryStore/attach_embedder.md)
- [default](../../../functions/src/memory/semantic/EmbedRuntime/default/default.md)
- [insert_at](../../../functions/tests/memory_consolidate_test/insert_at.md)
- [merge_band_vectors](../../../functions/tests/memory_consolidate_test/merge_band_vectors.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)