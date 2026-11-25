---
type: Rust Method
title: start_llama_cpp_delete
resource: src/tui/app.rs#L3677-L3690
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_llama_cpp_models_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn start_llama_cpp_delete(&mut self, path: std::path::PathBuf)`

# Calls

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)

# Called by

- [handle_llama_cpp_models_key](../../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)