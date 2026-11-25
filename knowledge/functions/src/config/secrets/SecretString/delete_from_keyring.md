---
type: Rust Method
title: delete_from_keyring
resource: src/config/secrets.rs#L92-L102
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_keyring
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/ProviderSecrets/delete_from_keyring
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn delete_from_keyring(key_name: &str) -> Result<()>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)
- [delete_from_keyring](../../../../../functions/src/config/secrets/ProviderSecrets/delete_from_keyring.md)