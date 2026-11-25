---
type: Rust Function
title: list_all_local_models_does_not_pair_across_directories
resource: src/llm/provider/llama_cpp_models.rs#L1995-L2006
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_all_local_models_does_not_pair_across_directories()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [all](../../../../../functions/src/memory/MemoryStore/all.md)