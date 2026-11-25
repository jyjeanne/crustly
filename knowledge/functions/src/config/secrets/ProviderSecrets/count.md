---
type: Rust Method
title: count
resource: src/config/secrets.rs#L287-L302
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/secrets/test_provider_secrets_empty
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/test_provider_secrets_with_keys
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn count(&self) -> usize`

# Called by

- [test_provider_secrets_empty](../../../../../functions/src/config/secrets/test_provider_secrets_empty.md)
- [test_provider_secrets_with_keys](../../../../../functions/src/config/secrets/test_provider_secrets_with_keys.md)