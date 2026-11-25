---
type: Rust Function
title: execute_is_atomic_across_files_on_failure
resource: src/llm/tools/apply_patch.rs#L792-L834
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/record
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/FileFingerprint/of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute_is_atomic_across_files_on_failure()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [record](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [of](../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)