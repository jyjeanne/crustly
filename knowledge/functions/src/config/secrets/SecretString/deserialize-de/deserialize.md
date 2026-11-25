---
type: Rust Method
title: deserialize
resource: src/config/secrets.rs#L179-L185
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/secrets/SecretString/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>,`

# Calls

- [new](../../../../../../functions/src/config/secrets/SecretString/new.md)