---
type: Rust Function
title: a_path_memory_is_shown_once_per_session
resource: tests/memory_paths_test.rs#L123-L167
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_paths_test/store_and_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/save_in_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/mark_shown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_path_memory_is_shown_once_per_session()`

# Calls

- [store_and_project](../../../functions/tests/memory_paths_test/store_and_project.md)
- [save_in_project](../../../functions/tests/memory_paths_test/save_in_project.md)
- [memories_for_path](../../../functions/src/memory/MemoryStore/memories_for_path.md)
- [mark_shown](../../../functions/src/memory/MemoryStore/mark_shown.md)