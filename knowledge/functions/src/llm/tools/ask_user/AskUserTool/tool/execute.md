---
type: Rust Method
title: execute
resource: src/llm/tools/ask_user.rs#L77-L130
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/tools/trait/ToolResult/with_metadata
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)