---
type: Rust Method
title: execute
resource: src/llm/tools/todo_write.rs#L220-L276
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/todo_write/TodoStore/load
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/todo_write/render_todos
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/todo_write/TodoStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [load](../../../../../../../functions/src/llm/tools/todo_write/TodoStore/load.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [render_todos](../../../../../../../functions/src/llm/tools/todo_write/render_todos.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [save](../../../../../../../functions/src/llm/tools/todo_write/TodoStore/save.md)