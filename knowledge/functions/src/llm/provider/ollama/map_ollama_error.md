---
type: Rust Function
title: map_ollama_error
resource: src/llm/provider/ollama.rs#L995-L1036
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/test_map_ollama_error_not_found
  - functions/src/llm/provider/ollama/model_not_found_error_is_unwrapped_and_actionable
---

# Signature

`fn map_ollama_error(err: OllamaError) -> ProviderError`

# Called by

- [test_map_ollama_error_not_found](../../../../../functions/src/llm/provider/ollama/test_map_ollama_error_not_found.md)
- [model_not_found_error_is_unwrapped_and_actionable](../../../../../functions/src/llm/provider/ollama/model_not_found_error_is_unwrapped_and_actionable.md)