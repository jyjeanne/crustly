---
type: Rust Function
title: tool_paths_match_when_absolute_or_relative_to_a_subdirectory
resource: tests/memory_paths_test.rs#L195-L246
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
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn tool_paths_match_when_absolute_or_relative_to_a_subdirectory()`

# Calls

- [store_and_real_project](../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [save_in_project](../../../functions/tests/memory_paths_test/save_in_project.md)
- [memories_for_path](../../../functions/src/memory/MemoryStore/memories_for_path.md)