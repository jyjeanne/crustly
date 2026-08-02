---
type: Rust Method
title: start_llama_cpp_switch
resource: src/tui/app.rs#L2826-L2841
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/event_sender
  - functions/src/tui/llama_cpp_download/spawn_switch
  called_by:
  - functions/src/tui/app/App/handle_llama_cpp_models_key
---

# Signature

`async fn start_llama_cpp_switch(&mut self, path: std::path::PathBuf)`

# Calls

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [spawn_switch](../../../../../functions/src/tui/llama_cpp_download/spawn_switch.md)

# Called by

- [handle_llama_cpp_models_key](../../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)