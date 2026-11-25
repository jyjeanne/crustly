---
type: Rust Method
title: from_str
resource: src/config/secrets.rs#L33-L37
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_keyring
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/from-str/from
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/test_secret_string_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/test_secret_string_debug
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/test_secret_string_display
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/test_provider_secrets_with_keys
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/test_secret_string_serialize
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn from_str(value: &str) -> Self`

# Called by

- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)
- [from](../../../../../functions/src/config/secrets/SecretString/from-str/from.md)
- [test_secret_string_creation](../../../../../functions/src/config/secrets/test_secret_string_creation.md)
- [test_secret_string_debug](../../../../../functions/src/config/secrets/test_secret_string_debug.md)
- [test_secret_string_display](../../../../../functions/src/config/secrets/test_secret_string_display.md)
- [test_provider_secrets_with_keys](../../../../../functions/src/config/secrets/test_provider_secrets_with_keys.md)
- [test_secret_string_serialize](../../../../../functions/src/config/secrets/test_secret_string_serialize.md)