---
type: Rust Method
title: from_keyring_optional
resource: src/config/secrets.rs#L67-L72
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/cli/cmd_keyring
  - functions/src/config/secrets/SecretString/load_with_fallback
---

# Signature

`pub fn from_keyring_optional(key_name: &str) -> Option<Self>`

# Called by

- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)
- [load_with_fallback](../../../../../functions/src/config/secrets/SecretString/load_with_fallback.md)