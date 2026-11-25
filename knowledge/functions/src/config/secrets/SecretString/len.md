---
type: Rust Method
title: len
resource: src/config/secrets.rs#L149-L151
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_keyring
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/secrets/test_secret_string_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn len(&self) -> usize`

# Called by

- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)
- [test_secret_string_creation](../../../../../functions/src/config/secrets/test_secret_string_creation.md)