---
type: Rust Function
title: a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget
resource: tests/memory_paths_test.rs#L77-L120
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_paths_test/store_and_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/save_in_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget()`

# Calls

- [store_and_project](../../../functions/tests/memory_paths_test/store_and_project.md)
- [save_in_project](../../../functions/tests/memory_paths_test/save_in_project.md)
- [recall](../../../functions/src/memory/MemoryStore/recall.md)