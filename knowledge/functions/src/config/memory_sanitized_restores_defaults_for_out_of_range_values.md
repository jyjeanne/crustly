---
type: Rust Function
title: memory_sanitized_restores_defaults_for_out_of_range_values
resource: src/config/mod.rs#L2384-L2393
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn memory_sanitized_restores_defaults_for_out_of_range_values()`

# Calls

- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [sanitized](../../../functions/src/config/MemoryConfig/sanitized.md)