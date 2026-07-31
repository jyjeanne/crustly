---
type: Rust Function
title: is_paste_clipboard
resource: src/tui/events.rs#L326-L328
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/key_matches
  called_by:
  - functions/src/tui/app/App/handle_chat_key
---

# Signature

`pub fn is_paste_clipboard(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)