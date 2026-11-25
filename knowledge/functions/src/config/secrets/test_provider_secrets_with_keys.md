---
type: Rust Function
title: test_provider_secrets_with_keys
resource: src/config/secrets.rs#L347-L356
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
  - target: functions/src/config/secrets/ProviderSecrets/has_any
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/ProviderSecrets/count
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_provider_secrets_with_keys()`

# Calls

- [from_str](../../../../functions/src/config/secrets/SecretString/from_str.md)
- [has_any](../../../../functions/src/config/secrets/ProviderSecrets/has_any.md)
- [count](../../../../functions/src/config/secrets/ProviderSecrets/count.md)