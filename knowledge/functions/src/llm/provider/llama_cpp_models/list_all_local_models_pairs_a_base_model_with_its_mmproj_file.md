---
type: Rust Function
title: list_all_local_models_pairs_a_base_model_with_its_mmproj_file
resource: src/llm/provider/llama_cpp_models.rs#L1925-L1953
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
---

# Signature

`fn list_all_local_models_pairs_a_base_model_with_its_mmproj_file()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)