---
type: Rust Function
title: up_recalls_previous_messages_without_sending_them
resource: src/tui/app.rs#L3315-L3343
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/push_input_history
  - functions/src/tui/app/App/set_input_text
  - functions/src/tui/app/App/handle_chat_key
  - functions/src/tui/app/key
---

# Signature

`async fn up_recalls_previous_messages_without_sending_them()`

# Calls

- [push_input_history](../../../../functions/src/tui/app/App/push_input_history.md)
- [set_input_text](../../../../functions/src/tui/app/App/set_input_text.md)
- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)