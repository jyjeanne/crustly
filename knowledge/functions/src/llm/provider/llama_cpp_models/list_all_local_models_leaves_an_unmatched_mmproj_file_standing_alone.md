---
type: Rust Function
title: list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone
resource: src/llm/provider/llama_cpp_models.rs#L1956-L1975
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
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)