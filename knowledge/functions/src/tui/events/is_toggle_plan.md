---
type: Rust Function
title: is_toggle_plan
resource: src/tui/events.rs#L298-L300
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/key_matches
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`pub fn is_toggle_plan(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)