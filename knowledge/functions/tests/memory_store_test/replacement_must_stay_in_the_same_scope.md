---
type: Rust Function
title: replacement_must_stay_in_the_same_scope
resource: tests/memory_store_test.rs#L196-L217
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/save_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn replacement_must_stay_in_the_same_scope()`

# Calls

- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [save_request](../../../functions/tests/memory_store_test/save_request.md)