---
type: Rust Function
title: test_secret_string_creation
resource: src/config/secrets.rs#L316-L321
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/secrets/SecretString/from_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/expose_secret
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/len
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/SecretString/is_empty
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_secret_string_creation()`

# Calls

- [from_str](../../../../functions/src/config/secrets/SecretString/from_str.md)
- [expose_secret](../../../../functions/src/config/secrets/SecretString/expose_secret.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)