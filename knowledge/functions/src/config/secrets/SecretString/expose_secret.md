---
type: Rust Method
title: expose_secret
resource: src/config/secrets.rs#L139-L141
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_keyring
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/save_to_keyring
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/test_secret_string_creation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/test_secret_string_from_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/test_secret_string_from_env_optional
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn expose_secret(&self) -> &str`

# Called by

- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)
- [save_to_keyring](../../../../../functions/src/config/secrets/SecretString/save_to_keyring.md)
- [test_secret_string_creation](../../../../../functions/src/config/secrets/test_secret_string_creation.md)
- [test_secret_string_from_env](../../../../../functions/src/config/secrets/test_secret_string_from_env.md)
- [test_secret_string_from_env_optional](../../../../../functions/src/config/secrets/test_secret_string_from_env_optional.md)