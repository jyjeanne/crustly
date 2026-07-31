---
type: Rust Method
title: start_model_delete
resource: src/tui/app.rs#L2277-L2288
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/event_sender
  called_by:
  - functions/src/tui/app/App/handle_model_download_key
---

# Signature

`async fn start_model_delete(&mut self, model: String)`

# Calls

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)

# Called by

- [handle_model_download_key](../../../../../functions/src/tui/app/App/handle_model_download_key.md)