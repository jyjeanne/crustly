---
type: Rust Function
title: test_event_handler_creation
resource: src/tui/events.rs#L597-L602
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/sender
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_event_handler_creation()`

# Calls

- [new](../../../../functions/src/tui/events/EventHandler/new.md)
- [sender](../../../../functions/src/tui/events/EventHandler/sender.md)