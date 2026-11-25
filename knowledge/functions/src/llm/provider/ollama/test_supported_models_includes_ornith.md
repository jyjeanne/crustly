---
type: Rust Function
title: test_supported_models_includes_ornith
resource: src/llm/provider/ollama.rs#L1099-L1103
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_supported_models_includes_ornith()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [supported_models](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/supported_models.md)