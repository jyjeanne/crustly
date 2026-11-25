---
type: Rust Function
title: recall_is_none_when_nothing_is_stored
resource: tests/memory_store_test.rs#L299-L308
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recall_is_none_when_nothing_is_stored()`

# Calls

- [recall](../../../functions/src/memory/MemoryStore/recall.md)
- [request](../../../functions/tests/memory_store_test/request.md)