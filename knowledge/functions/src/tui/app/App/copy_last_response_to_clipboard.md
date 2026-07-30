---
type: Rust Method
title: copy_last_response_to_clipboard
resource: src/tui/app.rs#L404-L415
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/last_assistant_message
  - functions/src/tui/markdown/last_code_block
  called_by:
  - functions/src/tui/app/App/handle_chat_key
---

# Signature

`fn copy_last_response_to_clipboard(&mut self)`

# Calls

- [last_assistant_message](../../../../../functions/src/tui/app/App/last_assistant_message.md)
- [last_code_block](../../../../../functions/src/tui/markdown/last_code_block.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)