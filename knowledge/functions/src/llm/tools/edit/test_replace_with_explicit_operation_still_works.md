---
type: Rust Function
title: test_replace_with_explicit_operation_still_works
resource: src/llm/tools/edit.rs#L443-L463
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

`async fn test_replace_with_explicit_operation_still_works()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [execute](../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)