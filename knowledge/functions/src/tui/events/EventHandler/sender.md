---
type: Rust Method
title: sender
resource: src/tui/events.rs#L198-L200
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/app/App/event_sender
  - functions/src/tui/events/test_event_handler_creation
---

# Signature

`pub fn sender(&self) -> mpsc::UnboundedSender<TuiEvent>`

# Called by

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [test_event_handler_creation](../../../../../functions/src/tui/events/test_event_handler_creation.md)