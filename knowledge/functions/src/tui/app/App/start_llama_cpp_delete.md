---
type: Rust Method
title: start_llama_cpp_delete
resource: src/tui/app.rs#L2807-L2820
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/event_sender
  called_by:
  - functions/src/tui/app/App/handle_llama_cpp_models_key
---

# Signature

`async fn start_llama_cpp_delete(&mut self, path: std::path::PathBuf)`

# Calls

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)

# Called by

- [handle_llama_cpp_models_key](../../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)