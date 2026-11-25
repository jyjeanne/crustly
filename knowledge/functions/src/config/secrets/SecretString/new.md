---
type: Rust Method
title: new
resource: src/config/secrets.rs#L27-L29
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/config/secrets/SecretString/from_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/from_keyring
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/deserialize-de/deserialize
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/from-string/from
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(value: String) -> Self`

# Called by

- [from_env](../../../../../functions/src/config/secrets/SecretString/from_env.md)
- [from_keyring](../../../../../functions/src/config/secrets/SecretString/from_keyring.md)
- [deserialize](../../../../../functions/src/config/secrets/SecretString/deserialize-de/deserialize.md)
- [from](../../../../../functions/src/config/secrets/SecretString/from-string/from.md)