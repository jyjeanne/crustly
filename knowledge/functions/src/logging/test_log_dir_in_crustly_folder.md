---
type: Rust Function
title: test_log_dir_in_crustly_folder
resource: src/logging.rs#L331-L336
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/logging/LogConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_log_dir_in_crustly_folder()`

# Calls

- [default](../../../functions/src/logging/LogConfig/default/default.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)