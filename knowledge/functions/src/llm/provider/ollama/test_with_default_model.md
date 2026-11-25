---
type: Rust Function
title: test_with_default_model
resource: src/llm/provider/ollama.rs#L987-L991
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
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_with_default_model()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_default_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_default_model.md)
- [default_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/default_model.md)