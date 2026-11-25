---
type: Rust Method
title: save
resource: src/llm/tools/todo_write.rs#L84-L90
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn save(&self, path: &Path) -> Result<()>`

# Calls

- [write](../../../../../../functions/src/memory/project/write.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute.md)