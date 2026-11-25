---
type: Rust Method
title: sender
resource: src/tui/events.rs#L240-L242
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/test_event_handler_creation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn sender(&self) -> mpsc::UnboundedSender<TuiEvent>`

# Called by

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [test_event_handler_creation](../../../../../functions/src/tui/events/test_event_handler_creation.md)