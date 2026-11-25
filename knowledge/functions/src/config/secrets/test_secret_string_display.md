---
type: Rust Function
title: test_secret_string_display
resource: src/config/secrets.rs#L332-L337
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
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_secret_string_display()`

# Calls

- [from_str](../../../../functions/src/config/secrets/SecretString/from_str.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)