---
type: Rust Method
title: history_next
resource: src/tui/app.rs#L445-L461
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/tui/app/App/load_history_entry
  called_by:
  - functions/src/tui/app/App/handle_chat_key
---

# Signature

`fn history_next(&mut self) -> bool`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [load_history_entry](../../../../../functions/src/tui/app/App/load_history_entry.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)