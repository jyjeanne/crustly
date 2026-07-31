---
type: Rust Method
title: execute
resource: src/llm/tools/code_exec.rs#L128-L265
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/error/CrustlyError/code
  - functions/src/config/secrets/SecretString/is_empty
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [code](../../../../../../../functions/src/error/CrustlyError/code.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)