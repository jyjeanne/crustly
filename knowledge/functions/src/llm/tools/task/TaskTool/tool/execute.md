---
type: Rust Method
title: execute
resource: src/llm/tools/task.rs#L388-L713
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/task/FileLock/acquire
  - functions/src/llm/tools/task/FileLock/release
  - functions/src/llm/tools/task/parse_status
  - functions/src/llm/tools/task/parse_priority
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/tools/task/TaskStore/with_lock
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [acquire](../../../../../../../functions/src/llm/tools/task/FileLock/acquire.md)
- [release](../../../../../../../functions/src/llm/tools/task/FileLock/release.md)
- [parse_status](../../../../../../../functions/src/llm/tools/task/parse_status.md)
- [parse_priority](../../../../../../../functions/src/llm/tools/task/parse_priority.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [with_lock](../../../../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)