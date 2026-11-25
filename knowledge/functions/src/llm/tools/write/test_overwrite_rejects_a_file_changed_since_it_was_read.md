---
type: Rust Function
title: test_overwrite_rejects_a_file_changed_since_it_was_read
resource: src/llm/tools/write.rs#L399-L428
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
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/record
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/FileFingerprint/of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_overwrite_rejects_a_file_changed_since_it_was_read()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_working_directory](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory.md)
- [record](../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [of](../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [execute](../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)