---
type: Rust Function
title: a_secret_is_rejected_and_nothing_is_stored
resource: tests/memory_store_test.rs#L126-L145
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_store_test/save_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/take_notices
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_secret_is_rejected_and_nothing_is_stored()`

# Calls

- [save_request](../../../functions/tests/memory_store_test/save_request.md)
- [take_notices](../../../functions/src/memory/MemoryStore/take_notices.md)