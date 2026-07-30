---
type: Rust Function
title: is_copy_response
resource: src/tui/events.rs#L319-L321
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/key_matches
  called_by:
  - functions/src/tui/app/App/handle_chat_key
---

# Signature

`pub fn is_copy_response(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)