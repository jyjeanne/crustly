---
type: Rust Function
title: llama_cpp_delete_finished_removes_model_from_list
resource: src/tui/app.rs#L5554-L5584
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn llama_cpp_delete_finished_removes_model_from_list()`

# Calls

- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)