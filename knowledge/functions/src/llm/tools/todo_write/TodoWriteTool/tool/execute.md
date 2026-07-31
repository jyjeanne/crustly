---
type: Rust Method
title: execute
resource: src/llm/tools/todo_write.rs#L220-L276
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/todo_write/render_todos
  - functions/src/llm/tools/trait/ToolResult/with_metadata
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [render_todos](../../../../../../../functions/src/llm/tools/todo_write/render_todos.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)