---
type: Rust Function
title: test_secret_string_from_env_optional
resource: src/config/secrets.rs#L371-L384
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/secrets/SecretString/from_env_optional
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/expose_secret
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_secret_string_from_env_optional()`

# Calls

- [from_env_optional](../../../../functions/src/config/secrets/SecretString/from_env_optional.md)
- [expose_secret](../../../../functions/src/config/secrets/SecretString/expose_secret.md)