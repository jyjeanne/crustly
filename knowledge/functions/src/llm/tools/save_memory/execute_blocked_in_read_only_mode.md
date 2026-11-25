---
type: Rust Function
title: execute_blocked_in_read_only_mode
resource: src/llm/tools/save_memory.rs#L450-L458
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

`async fn execute_blocked_in_read_only_mode()`

# Calls

- [file_only](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/file_only.md)
- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)