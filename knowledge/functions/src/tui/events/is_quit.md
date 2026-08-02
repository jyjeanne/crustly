---
type: Rust Function
title: is_quit
resource: src/tui/events.rs#L305-L307
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/key_matches
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`pub fn is_quit(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)