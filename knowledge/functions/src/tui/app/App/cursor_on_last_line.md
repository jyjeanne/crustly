---
type: Rust Method
title: cursor_on_last_line
resource: src/tui/app.rs#L340-L342
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/tui/app/App/handle_chat_key
---

# Signature

`fn cursor_on_last_line(&self) -> bool`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)