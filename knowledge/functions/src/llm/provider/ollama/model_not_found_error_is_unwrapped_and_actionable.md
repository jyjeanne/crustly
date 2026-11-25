---
type: Rust Function
title: model_not_found_error_is_unwrapped_and_actionable
resource: src/llm/provider/ollama.rs#L1176-L1191
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/map_ollama_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn model_not_found_error_is_unwrapped_and_actionable()`

# Calls

- [map_ollama_error](../../../../../functions/src/llm/provider/ollama/map_ollama_error.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)