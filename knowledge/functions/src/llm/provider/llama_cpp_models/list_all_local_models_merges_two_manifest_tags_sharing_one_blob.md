---
type: Rust Function
title: list_all_local_models_merges_two_manifest_tags_sharing_one_blob
resource: src/llm/provider/llama_cpp_models.rs#L1712-L1749
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/write_fake_ollama_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_all_local_models_merges_two_manifest_tags_sharing_one_blob()`

# Calls

- [write_fake_ollama_model](../../../../../functions/src/llm/provider/llama_cpp_models/write_fake_ollama_model.md)
- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)