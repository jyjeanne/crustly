---
type: Rust Function
title: render_todos
resource: src/llm/tools/todo_write.rs#L126-L150
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute
  - functions/src/llm/tools/todo_write/test_render_todos_completed
---

# Signature

`fn render_todos(todos: &[TodoItem]) -> String`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/todo_write/TodoWriteTool/tool/execute.md)
- [test_render_todos_completed](../../../../../functions/src/llm/tools/todo_write/test_render_todos_completed.md)