---
type: Rust Function
title: test_edit_rejects_a_file_changed_since_it_was_read
resource: src/llm/tools/edit.rs#L650-L677
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`async fn test_edit_rejects_a_file_changed_since_it_was_read()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [execute](../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)