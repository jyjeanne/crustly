---
type: Rust Method
title: save_to_keyring
resource: src/config/secrets.rs#L79-L89
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/secrets/SecretString/expose_secret
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_keyring
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn save_to_keyring(&self, key_name: &str) -> Result<()>`

# Calls

- [expose_secret](../../../../../functions/src/config/secrets/SecretString/expose_secret.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)