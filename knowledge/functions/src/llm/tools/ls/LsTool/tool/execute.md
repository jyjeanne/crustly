---
type: Rust Method
title: execute
resource: src/llm/tools/ls.rs#L86-L134
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/llm/tools/ls/LsTool/list_recursive
  - functions/src/llm/tools/ls/LsTool/list_directory
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [list_recursive](../../../../../../../functions/src/llm/tools/ls/LsTool/list_recursive.md)
- [list_directory](../../../../../../../functions/src/llm/tools/ls/LsTool/list_directory.md)