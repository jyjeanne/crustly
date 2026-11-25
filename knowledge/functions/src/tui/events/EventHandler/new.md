---
type: Rust Method
title: new
resource: src/tui/events.rs#L234-L237
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/test_event_handler_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Called by

- [new](../../../../../functions/src/tui/app/App/new.md)
- [default](../../../../../functions/src/tui/events/EventHandler/default/default.md)
- [test_event_handler_creation](../../../../../functions/src/tui/events/test_event_handler_creation.md)