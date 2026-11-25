---
type: Rust Function
title: recall_includes_own_project_and_global_only
resource: tests/memory_store_test.rs#L245-L296
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
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recall_includes_own_project_and_global_only()`

# Calls

- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [save_request](../../../functions/tests/memory_store_test/save_request.md)
- [recall](../../../functions/src/memory/MemoryStore/recall.md)
- [request](../../../functions/tests/memory_store_test/request.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)