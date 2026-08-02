---
type: Rust Method
title: open_file_picker
resource: src/tui/app.rs#L2402-L2435
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/switch_mode
  called_by:
  - functions/src/tui/app/App/handle_chat_key
  - functions/src/tui/app/App/handle_file_picker_key
---

# Signature

`async fn open_file_picker(&mut self) -> Result<()>`

# Calls

- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [handle_file_picker_key](../../../../../functions/src/tui/app/App/handle_file_picker_key.md)