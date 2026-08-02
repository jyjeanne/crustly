---
type: Rust Method
title: validate_input
resource: src/llm/tools/powershell.rs#L220-L237
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
---

# Signature

`fn validate_input(&self, input: &Value) -> Result<()>`

# Calls

- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)