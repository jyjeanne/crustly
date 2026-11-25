---
type: Rust Method
title: from_keyring
resource: src/config/secrets.rs#L55-L64
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
---

# Signature

`pub fn from_keyring(key_name: &str) -> Result<Self>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [new](../../../../../functions/src/config/secrets/SecretString/new.md)