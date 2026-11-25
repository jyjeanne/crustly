---
type: Rust Method
title: load_with_fallback
resource: src/config/secrets.rs#L110-L132
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/secrets/SecretString/from_keyring_optional
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/from_env_optional
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/secrets/ProviderSecrets/load_with_fallback
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn load_with_fallback(key_name: &str, env_var: &str) -> Option<Self>`

# Calls

- [from_keyring_optional](../../../../../functions/src/config/secrets/SecretString/from_keyring_optional.md)
- [from_env_optional](../../../../../functions/src/config/secrets/SecretString/from_env_optional.md)

# Called by

- [load_with_fallback](../../../../../functions/src/config/secrets/ProviderSecrets/load_with_fallback.md)