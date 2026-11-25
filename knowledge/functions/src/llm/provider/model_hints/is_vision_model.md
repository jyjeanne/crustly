---
type: Rust Function
title: is_vision_model
resource: src/llm/provider/model_hints.rs#L10-L28
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/model_hints/detects_known_vision_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/model_hints/rejects_non_vision_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/supports_vision
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/supports_vision
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_vision_model(model_name: &str) -> bool`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [detects_known_vision_models](../../../../../functions/src/llm/provider/model_hints/detects_known_vision_models.md)
- [rejects_non_vision_models](../../../../../functions/src/llm/provider/model_hints/rejects_non_vision_models.md)
- [supports_vision](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/supports_vision.md)
- [supports_vision](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/supports_vision.md)