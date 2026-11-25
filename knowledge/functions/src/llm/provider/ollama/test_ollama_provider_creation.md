---
type: Rust Function
title: test_ollama_provider_creation
resource: src/llm/provider/ollama.rs#L980-L984
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
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_ollama_provider_creation()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [name](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/name.md)
- [default_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/default_model.md)