---
type: Rust Function
title: list_ollama_models_skips_a_manifest_whose_blob_is_missing
resource: src/llm/provider/llama_cpp_models.rs#L1690-L1709
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
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_ollama_models_skips_a_manifest_whose_blob_is_missing()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [list_ollama_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models.md)