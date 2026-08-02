---
type: Rust Method
title: open_llama_cpp_models
resource: src/tui/app.rs#L2767-L2785
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/event_sender
  - functions/src/tui/app/App/switch_mode
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`async fn open_llama_cpp_models(&mut self) -> Result<()>`

# Calls

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)