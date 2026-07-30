---
type: Rust Function
title: is_vision_model
resource: src/llm/provider/model_hints.rs#L10-L28
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/provider/supports_vision
  - functions/src/llm/provider/openai/OpenAIProvider/provider/supports_vision
---

# Signature

`pub fn is_vision_model(model_name: &str) -> bool`

# Called by

- [supports_vision](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/supports_vision.md)
- [supports_vision](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/supports_vision.md)