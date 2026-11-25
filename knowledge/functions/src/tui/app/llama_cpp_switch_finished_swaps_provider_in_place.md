---
type: Rust Function
title: llama_cpp_switch_finished_swaps_provider_in_place
resource: src/tui/app.rs#L5594-L5615
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn llama_cpp_switch_finished_swaps_provider_in_place()`

# Calls

- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [provider_name](../../../../functions/src/tui/app/App/provider_name.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)