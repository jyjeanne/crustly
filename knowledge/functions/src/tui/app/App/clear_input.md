---
type: Rust Method
title: clear_input
resource: src/tui/app.rs#L307-L309
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/plain_textarea
  called_by:
  - functions/src/tui/app/App/handle_chat_key
  - functions/src/tui/app/chat_input_text_is_not_underlined
---

# Signature

`fn clear_input(&mut self)`

# Calls

- [plain_textarea](../../../../../functions/src/tui/app/plain_textarea.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [chat_input_text_is_not_underlined](../../../../../functions/src/tui/app/chat_input_text_is_not_underlined.md)