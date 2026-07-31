---
type: Rust Method
title: execute
resource: src/llm/tools/powershell.rs#L239-L364
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/powershell/is_read_only_powershell
  - functions/src/llm/tools/task/FileLock/drop/drop
  - functions/src/llm/tools/trait/ToolResult/with_metadata
  - functions/src/error/CrustlyError/code
  - functions/src/config/secrets/SecretString/is_empty
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_read_only_powershell](../../../../../../../functions/src/llm/tools/powershell/is_read_only_powershell.md)
- [drop](../../../../../../../functions/src/llm/tools/task/FileLock/drop/drop.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [code](../../../../../../../functions/src/error/CrustlyError/code.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)