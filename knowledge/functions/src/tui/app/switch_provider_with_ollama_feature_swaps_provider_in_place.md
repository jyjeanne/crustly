---
type: Rust Function
title: switch_provider_with_ollama_feature_swaps_provider_in_place
resource: src/tui/app.rs#L5316-L5334
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/switch_provider_to_ollama_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/provider_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn switch_provider_with_ollama_feature_swaps_provider_in_place()`

# Calls

- [provider_name](../../../../functions/src/tui/app/App/provider_name.md)
- [switch_provider_to_ollama_model](../../../../functions/src/tui/app/App/switch_provider_to_ollama_model.md)
- [provider_model](../../../../functions/src/tui/app/App/provider_model.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)