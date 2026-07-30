---
type: Rust Method
title: validate_input
resource: src/llm/tools/grep.rs#L128-L139
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
---

# Signature

`fn validate_input(&self, input: &Value) -> Result<()>`

# Calls

- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)