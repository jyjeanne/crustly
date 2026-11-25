---
type: Rust Function
title: test_secret_string_from_env
resource: src/config/secrets.rs#L359-L368
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/secrets/SecretString/from_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/expose_secret
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_secret_string_from_env()`

# Calls

- [from_env](../../../../functions/src/config/secrets/SecretString/from_env.md)
- [expose_secret](../../../../functions/src/config/secrets/SecretString/expose_secret.md)