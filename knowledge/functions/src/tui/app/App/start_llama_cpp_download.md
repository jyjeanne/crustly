---
type: Rust Method
title: start_llama_cpp_download
resource: src/tui/app.rs#L2789-L2803
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/app/App/event_sender
  called_by:
  - functions/src/tui/app/App/handle_llama_cpp_models_key
---

# Signature

`async fn start_llama_cpp_download(&mut self)`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)

# Called by

- [handle_llama_cpp_models_key](../../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)