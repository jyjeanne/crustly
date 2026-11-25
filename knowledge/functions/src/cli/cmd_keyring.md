---
type: Rust Function
title: cmd_keyring
resource: src/cli/mod.rs#L2360-L2455
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/secrets/SecretString/from_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/save_to_keyring
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/from_keyring_optional
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/delete_from_keyring
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/expose_secret
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_keyring(operation: KeyringCommands) -> Result<()>`

# Calls

- [from_str](../../../functions/src/config/secrets/SecretString/from_str.md)
- [save_to_keyring](../../../functions/src/config/secrets/SecretString/save_to_keyring.md)
- [with_context](../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [from_keyring_optional](../../../functions/src/config/secrets/SecretString/from_keyring_optional.md)
- [delete_from_keyring](../../../functions/src/config/secrets/SecretString/delete_from_keyring.md)
- [expose_secret](../../../functions/src/config/secrets/SecretString/expose_secret.md)

# Called by

- [run](../../../functions/src/cli/run.md)