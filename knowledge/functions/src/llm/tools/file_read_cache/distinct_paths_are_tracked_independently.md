---
type: Rust Function
title: distinct_paths_are_tracked_independently
resource: src/llm/tools/file_read_cache.rs#L127-L131
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/record
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/fp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/check
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn distinct_paths_are_tracked_independently()`

# Calls

- [new](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/new.md)
- [record](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [fp](../../../../../functions/src/llm/tools/file_read_cache/fp.md)
- [check](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/check.md)