---
type: Rust Method
title: from_env
resource: src/config/secrets.rs#L40-L44
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/config/secrets/test_secret_string_from_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn from_env(var_name: &str) -> Result<Self>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [new](../../../../../functions/src/config/secrets/SecretString/new.md)

# Called by

- [test_secret_string_from_env](../../../../../functions/src/config/secrets/test_secret_string_from_env.md)