---
type: Rust Function
title: re_recording_updates_the_fingerprint
resource: src/llm/tools/file_read_cache.rs#L134-L140
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`fn re_recording_updates_the_fingerprint()`

# Calls

- [new](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/new.md)
- [record](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [fp](../../../../../functions/src/llm/tools/file_read_cache/fp.md)
- [check](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/check.md)