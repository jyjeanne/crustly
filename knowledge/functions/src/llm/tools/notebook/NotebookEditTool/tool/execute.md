---
type: Rust Method
title: execute
resource: src/llm/tools/notebook.rs#L161-L335
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/check_path
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [from_str](../../../../../../../functions/src/config/secrets/SecretString/from_str.md)
- [len](../../../../../../../functions/src/config/secrets/SecretString/len.md)