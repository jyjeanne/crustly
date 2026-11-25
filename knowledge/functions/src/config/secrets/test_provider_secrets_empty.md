---
type: Rust Function
title: test_provider_secrets_empty
resource: src/config/secrets.rs#L340-L344
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/secrets/ProviderSecrets/new
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

`fn test_provider_secrets_empty()`

# Calls

- [new](../../../../functions/src/config/secrets/ProviderSecrets/new.md)
- [has_any](../../../../functions/src/config/secrets/ProviderSecrets/has_any.md)
- [count](../../../../functions/src/config/secrets/ProviderSecrets/count.md)