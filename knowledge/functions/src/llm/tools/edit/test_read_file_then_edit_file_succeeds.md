---
type: Rust Function
title: test_read_file_then_edit_file_succeeds
resource: src/llm/tools/edit.rs#L683-L708
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
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_read_file_then_edit_file_succeeds()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [execute](../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)