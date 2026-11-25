---
type: Rust Function
title: estimate_default_context_length
resource: src/llm/provider/llama_cpp_models.rs#L45-L49
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn estimate_default_context_length(native: Option<u64>) -> u64`

# Called by

- [list_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)
- [parse_ollama_manifest](../../../../../functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest.md)