---
type: Rust Method
title: start_model_pull
resource: src/tui/app.rs#L2260-L2273
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/app/App/event_sender
  called_by:
  - functions/src/tui/app/App/handle_model_download_key
---

# Signature

`async fn start_model_pull(&mut self, model: String)`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)

# Called by

- [handle_model_download_key](../../../../../functions/src/tui/app/App/handle_model_download_key.md)