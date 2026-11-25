---
type: Rust Function
title: list_local_models_only_lists_gguf_files
resource: src/llm/provider/llama_cpp_models.rs#L1247-L1257
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
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_local_models_only_lists_gguf_files()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)