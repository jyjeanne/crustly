---
type: Rust Function
title: llama_cpp_delete_key_asks_for_confirmation_before_deleting
resource: src/tui/app.rs#L5522-L5551
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_llama_cpp_models_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn llama_cpp_delete_key_asks_for_confirmation_before_deleting()`

# Calls

- [handle_llama_cpp_models_key](../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)
- [key](../../../../functions/src/tui/app/key.md)