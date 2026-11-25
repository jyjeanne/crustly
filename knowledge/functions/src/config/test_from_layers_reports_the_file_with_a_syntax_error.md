---
type: Rust Function
title: test_from_layers_reports_the_file_with_a_syntax_error
resource: src/config/mod.rs#L2100-L2112
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_from_layers_reports_the_file_with_a_syntax_error()`

# Calls

- [config_file](../../../functions/src/config/config_file.md)
- [from_layers](../../../functions/src/config/Config/from_layers.md)
- [layers](../../../functions/src/config/layers.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)