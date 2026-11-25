---
type: Rust Function
title: switch_provider_without_ollama_feature_shows_clear_error
resource: src/tui/app.rs#L5298-L5312
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/switch_provider_to_ollama_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn switch_provider_without_ollama_feature_shows_clear_error()`

# Calls

- [switch_provider_to_ollama_model](../../../../functions/src/tui/app/App/switch_provider_to_ollama_model.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)