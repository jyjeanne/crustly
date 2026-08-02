---
type: Rust Method
title: system
resource: src/llm/provider/types.rs#L47-L52
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
---

# Signature

`pub fn system(text: impl Into<String>) -> Self`

# Called by

- [to_ollama_request](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)