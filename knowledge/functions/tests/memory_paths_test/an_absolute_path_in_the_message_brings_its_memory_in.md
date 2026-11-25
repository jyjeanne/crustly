---
type: Rust Function
title: an_absolute_path_in_the_message_brings_its_memory_in
resource: tests/memory_paths_test.rs#L249-L285
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_paths_test/store_and_real_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/save_in_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_absolute_path_in_the_message_brings_its_memory_in()`

# Calls

- [store_and_real_project](../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [save_in_project](../../../functions/tests/memory_paths_test/save_in_project.md)
- [recall](../../../functions/src/memory/MemoryStore/recall.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)