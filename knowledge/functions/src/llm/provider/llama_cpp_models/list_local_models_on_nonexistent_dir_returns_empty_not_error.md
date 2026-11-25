---
type: Rust Function
title: list_local_models_on_nonexistent_dir_returns_empty_not_error
resource: src/llm/provider/llama_cpp_models.rs#L1240-L1244
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_local_models_on_nonexistent_dir_returns_empty_not_error()`

# Calls

- [list_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)