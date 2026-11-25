---
type: Rust Function
title: execute_creates_memory_file_with_header_and_fact
resource: src/llm/tools/save_memory.rs#L345-L365
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`async fn execute_creates_memory_file_with_header_and_fact()`

# Calls

- [file_only](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/file_only.md)
- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)