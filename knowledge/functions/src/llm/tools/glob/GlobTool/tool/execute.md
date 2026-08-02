---
type: Rust Method
title: execute
resource: src/llm/tools/glob.rs#L91-L191
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)