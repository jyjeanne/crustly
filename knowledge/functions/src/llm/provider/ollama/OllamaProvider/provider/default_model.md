---
type: Rust Method
title: default_model
resource: src/llm/provider/ollama.rs#L631-L633
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/supports_vision
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_ollama_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default_model(&self) -> &str`

# Called by

- [supports_vision](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/supports_vision.md)
- [test_ollama_provider_creation](../../../../../../../functions/src/llm/provider/ollama/test_ollama_provider_creation.md)
- [test_with_default_model](../../../../../../../functions/src/llm/provider/ollama/test_with_default_model.md)