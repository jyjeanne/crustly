---
type: Rust Method
title: expose_secret
resource: src/config/secrets.rs#L139-L141
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/config/secrets/SecretString/save_to_keyring
---

# Signature

`pub fn expose_secret(&self) -> &str`

# Called by

- [save_to_keyring](../../../../../functions/src/config/secrets/SecretString/save_to_keyring.md)