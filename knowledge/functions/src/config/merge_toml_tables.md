---
type: Rust Function
title: merge_toml_tables
resource: src/config/mod.rs#L1392-L1408
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/merge_toml_tables
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/merge_toml_tables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/from_layers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn merge_toml_tables(base: &mut toml::Table, overlay: toml::Table)`

# Calls

- [merge_toml_tables](../../../functions/src/config/merge_toml_tables.md)

# Called by

- [merge_toml_tables](../../../functions/src/config/merge_toml_tables.md)
- [from_layers](../../../functions/src/config/Config/from_layers.md)