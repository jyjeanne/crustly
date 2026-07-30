---
type: Rust Method
title: history_prev
resource: src/tui/app.rs#L356-L375
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/app/App/input_text
  - functions/src/config/secrets/SecretString/len
  - functions/src/tui/app/App/load_history_entry
  called_by:
  - functions/src/tui/app/App/handle_chat_key
---

# Signature

`fn history_prev(&mut self) -> bool`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [input_text](../../../../../functions/src/tui/app/App/input_text.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [load_history_entry](../../../../../functions/src/tui/app/App/load_history_entry.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)