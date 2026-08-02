---
type: Rust Function
title: is_submit
resource: src/tui/events.rs#L378-L381
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/app/App/handle_chat_key
---

# Signature

`pub fn is_submit(event: &KeyEvent) -> bool`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)