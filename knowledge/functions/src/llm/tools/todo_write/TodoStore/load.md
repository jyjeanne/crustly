---
type: Rust Method
title: load
resource: src/llm/tools/todo_write.rs#L75-L82
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn load(path: &Path) -> Result<Self>`

# Called by

- [execute](../../../../../../functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute.md)