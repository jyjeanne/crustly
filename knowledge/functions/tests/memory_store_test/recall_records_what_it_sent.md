---
type: Rust Function
title: recall_records_what_it_sent
resource: tests/memory_store_test.rs#L349-L371
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/save_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recall_records_what_it_sent()`

# Calls

- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [save_request](../../../functions/tests/memory_store_test/save_request.md)
- [recall](../../../functions/src/memory/MemoryStore/recall.md)
- [request](../../../functions/tests/memory_store_test/request.md)