---
type: Rust Function
title: llama_cpp_switch_is_a_noop_while_a_delete_is_already_running
resource: src/tui/app.rs#L5643-L5669
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`async fn llama_cpp_switch_is_a_noop_while_a_delete_is_already_running()`

# Calls

- [handle_llama_cpp_models_key](../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)
- [key](../../../../functions/src/tui/app/key.md)