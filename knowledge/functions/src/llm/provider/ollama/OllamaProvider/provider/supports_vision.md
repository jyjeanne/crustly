---
type: Rust Method
title: supports_vision
resource: src/llm/provider/ollama.rs#L623-L625
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/model_hints/is_vision_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn supports_vision(&self) -> bool`

# Calls

- [is_vision_model](../../../../../../../functions/src/llm/provider/model_hints/is_vision_model.md)
- [default_model](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/default_model.md)