---
type: Rust Method
title: is_connected
resource: src/db/mod.rs#L126-L128
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/test_connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/tilde_in_the_database_path_is_expanded_to_home
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn is_connected(&self) -> bool`

# Called by

- [test_connect_in_memory](../../../../functions/src/db/test_connect_in_memory.md)
- [tilde_in_the_database_path_is_expanded_to_home](../../../../functions/src/db/tilde_in_the_database_path_is_expanded_to_home.md)