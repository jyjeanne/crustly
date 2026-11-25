---
type: Rust Function
title: list_all_local_models_collapses_a_symlink_to_an_already_found_file
resource: src/llm/provider/llama_cpp_models.rs#L1755-L1772
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn list_all_local_models_collapses_a_symlink_to_an_already_found_file()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)