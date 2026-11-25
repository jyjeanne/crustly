---
type: Rust Function
title: an_expired_memory_is_archived_on_apply_and_only_reported_before
resource: tests/memory_consolidate_test.rs#L80-L104
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_consolidate_test/insert_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_expired_memory_is_archived_on_apply_and_only_reported_before()`

# Calls

- [insert_at](../../../functions/tests/memory_consolidate_test/insert_at.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)