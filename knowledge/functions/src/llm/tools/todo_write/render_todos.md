---
type: Rust Function
title: render_todos
resource: src/llm/tools/todo_write.rs#L126-L150
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/todo_write/test_render_todos_completed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/todo_write/test_render_todos_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_todos(todos: &[TodoItem]) -> String`

# Called by

- [execute](../../../../../functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute.md)
- [test_render_todos_completed](../../../../../functions/src/llm/tools/todo_write/test_render_todos_completed.md)
- [test_render_todos_empty](../../../../../functions/src/llm/tools/todo_write/test_render_todos_empty.md)