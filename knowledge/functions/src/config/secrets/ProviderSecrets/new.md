---
type: Rust Method
title: new
resource: src/config/secrets.rs#L213-L220
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/secrets/ProviderSecrets/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/test_provider_secrets_empty
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Called by

- [default](../../../../../functions/src/config/secrets/ProviderSecrets/default/default.md)
- [test_provider_secrets_empty](../../../../../functions/src/config/secrets/test_provider_secrets_empty.md)