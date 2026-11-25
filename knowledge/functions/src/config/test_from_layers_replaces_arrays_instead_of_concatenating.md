---
type: Rust Function
title: test_from_layers_replaces_arrays_instead_of_concatenating
resource: src/config/mod.rs#L2172-L2183
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/config_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/from_layers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/layers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_from_layers_replaces_arrays_instead_of_concatenating()`

# Calls

- [config_file](../../../functions/src/config/config_file.md)
- [from_layers](../../../functions/src/config/Config/from_layers.md)
- [layers](../../../functions/src/config/layers.md)