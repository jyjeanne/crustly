---
type: Rust Function
title: read_terminal_events
resource: src/tui/events.rs#L286-L337
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/is_text_press
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/coalesce_input
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/events/EventHandler/start_terminal_listener
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_terminal_events(tx: &mpsc::UnboundedSender<TuiEvent>)`

# Calls

- [is_text_press](../../../../functions/src/tui/events/is_text_press.md)
- [coalesce_input](../../../../functions/src/tui/events/coalesce_input.md)

# Called by

- [start_terminal_listener](../../../../functions/src/tui/events/EventHandler/start_terminal_listener.md)