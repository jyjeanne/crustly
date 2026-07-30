---
type: Rust Method
title: save_to_keyring
resource: src/config/secrets.rs#L79-L89
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/config/secrets/SecretString/expose_secret
---

# Signature

`pub fn save_to_keyring(&self, key_name: &str) -> Result<()>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [expose_secret](../../../../../functions/src/config/secrets/SecretString/expose_secret.md)