---
type: Rust Function
title: memory_persists_across_different_sessions_in_the_same_directory
resource: src/llm/tools/save_memory.rs#L414-L447
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/file_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn memory_persists_across_different_sessions_in_the_same_directory()`

# Calls

- [file_only](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/file_only.md)
- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)