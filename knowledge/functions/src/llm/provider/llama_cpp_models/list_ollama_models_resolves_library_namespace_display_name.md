---
type: Rust Function
title: list_ollama_models_resolves_library_namespace_display_name
resource: src/llm/provider/llama_cpp_models.rs#L1640-L1660
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/write_fake_ollama_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_ollama_models_resolves_library_namespace_display_name()`

# Calls

- [write_fake_ollama_model](../../../../../functions/src/llm/provider/llama_cpp_models/write_fake_ollama_model.md)
- [list_ollama_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models.md)