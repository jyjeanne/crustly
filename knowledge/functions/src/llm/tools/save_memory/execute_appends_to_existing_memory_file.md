---
type: Rust Function
title: execute_appends_to_existing_memory_file
resource: src/llm/tools/save_memory.rs#L368-L384
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
---

# Signature

`async fn execute_appends_to_existing_memory_file()`

# Calls

- [file_only](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/file_only.md)
- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)