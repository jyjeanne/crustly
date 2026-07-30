---
type: Rust Function
title: is_provider_switch
resource: src/tui/events.rs#L313-L315
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/key_matches
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`pub fn is_provider_switch(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)