---
type: Rust Method
title: from_env_optional
resource: src/config/secrets.rs#L47-L49
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/config/secrets/SecretString/load_with_fallback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/ProviderSecrets/from_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/test_secret_string_from_env_optional
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn from_env_optional(var_name: &str) -> Option<Self>`

# Called by

- [load_with_fallback](../../../../../functions/src/config/secrets/SecretString/load_with_fallback.md)
- [from_env](../../../../../functions/src/config/secrets/ProviderSecrets/from_env.md)
- [test_secret_string_from_env_optional](../../../../../functions/src/config/secrets/test_secret_string_from_env_optional.md)