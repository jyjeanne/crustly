---
type: Rust Function
title: test_system_config_path
resource: src/config/mod.rs#L2217-L2223
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/Config/system_config_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_system_config_path()`

# Calls

- [system_config_path](../../../functions/src/config/Config/system_config_path.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)