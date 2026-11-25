---
type: Rust Method
title: has_any
resource: src/config/secrets.rs#L279-L284
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/config/secrets/test_provider_secrets_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/test_provider_secrets_with_keys
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn has_any(&self) -> bool`

# Called by

- [test_provider_secrets_empty](../../../../../functions/src/config/secrets/test_provider_secrets_empty.md)
- [test_provider_secrets_with_keys](../../../../../functions/src/config/secrets/test_provider_secrets_with_keys.md)