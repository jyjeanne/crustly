---
type: Rust Method
title: load_with_fallback
resource: src/config/secrets.rs#L110-L132
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_keyring_optional
  - functions/src/config/secrets/SecretString/from_env_optional
---

# Signature

`pub fn load_with_fallback(key_name: &str, env_var: &str) -> Option<Self>`

# Calls

- [from_keyring_optional](../../../../../functions/src/config/secrets/SecretString/from_keyring_optional.md)
- [from_env_optional](../../../../../functions/src/config/secrets/SecretString/from_env_optional.md)