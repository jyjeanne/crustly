---
type: Rust Method
title: execute
resource: src/llm/tools/bash.rs#L309-L426
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/bash/is_read_only_command
  - functions/src/llm/tools/bash/resolve_shell
  - functions/src/error/CrustlyError/code
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/tools/trait/ToolResult/with_metadata
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_read_only_command](../../../../../../../functions/src/llm/tools/bash/is_read_only_command.md)
- [resolve_shell](../../../../../../../functions/src/llm/tools/bash/resolve_shell.md)
- [code](../../../../../../../functions/src/error/CrustlyError/code.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)