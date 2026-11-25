---
type: Rust Method
title: start_model_pull
resource: src/tui/app.rs#L3373-L3386
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_model_download_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn start_model_pull(&mut self, model: String)`

# Calls

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)

# Called by

- [handle_model_download_key](../../../../../functions/src/tui/app/App/handle_model_download_key.md)