---
type: Rust Function
title: cmd_keyring
resource: src/cli/mod.rs#L993-L1088
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/config/secrets/SecretString/from_keyring_optional
  called_by:
  - functions/src/cli/run
---

# Signature

`async fn cmd_keyring(operation: KeyringCommands) -> Result<()>`

# Calls

- [from_str](../../../functions/src/config/secrets/SecretString/from_str.md)
- [with_context](../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [from_keyring_optional](../../../functions/src/config/secrets/SecretString/from_keyring_optional.md)

# Called by

- [run](../../../functions/src/cli/run.md)