---
type: Rust Function
title: map_ollama_error
resource: src/llm/provider/ollama.rs#L932-L973
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/ollama/test_map_ollama_error_not_found
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/model_not_found_error_is_unwrapped_and_actionable
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn map_ollama_error(err: OllamaError) -> ProviderError`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [test_map_ollama_error_not_found](../../../../../functions/src/llm/provider/ollama/test_map_ollama_error_not_found.md)
- [model_not_found_error_is_unwrapped_and_actionable](../../../../../functions/src/llm/provider/ollama/model_not_found_error_is_unwrapped_and_actionable.md)