---
type: Rust Function
title: user_repeats_raise_importance_and_model_repeats_do_not
resource: tests/memory_store_test.rs#L87-L123
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
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn user_repeats_raise_importance_and_model_repeats_do_not()`

# Calls

- [save_request](../../../functions/tests/memory_store_test/save_request.md)
- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [get](../../../functions/src/db/repository/memory/MemoryRepository/get.md)