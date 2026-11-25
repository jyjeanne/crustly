---
type: Rust Function
title: is_text_press
resource: src/tui/events.rs#L409-L415
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/typed_char
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/events/read_terminal_events
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_text_press(event: &crossterm::event::Event) -> bool`

# Calls

- [typed_char](../../../../functions/src/tui/events/typed_char.md)

# Called by

- [read_terminal_events](../../../../functions/src/tui/events/read_terminal_events.md)