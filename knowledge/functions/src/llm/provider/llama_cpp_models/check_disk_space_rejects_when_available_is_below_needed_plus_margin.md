---
type: Rust Function
title: check_disk_space_rejects_when_available_is_below_needed_plus_margin
resource: src/llm/provider/llama_cpp_models.rs#L1293-L1302
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/check_disk_space
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_disk_space_rejects_when_available_is_below_needed_plus_margin()`

# Calls

- [check_disk_space](../../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)