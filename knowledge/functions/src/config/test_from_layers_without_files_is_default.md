---
type: Rust Function
title: test_from_layers_without_files_is_default
resource: src/config/mod.rs#L2186-L2193
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/Config/from_layers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_from_layers_without_files_is_default()`

# Calls

- [from_layers](../../../functions/src/config/Config/from_layers.md)
- [default](../../../functions/src/config/Config/default/default.md)