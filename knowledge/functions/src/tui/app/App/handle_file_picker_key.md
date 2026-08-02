---
type: Rust Method
title: handle_file_picker_key
resource: src/tui/app.rs#L2438-L2489
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/is_cancel
  - functions/src/tui/app/App/switch_mode
  - functions/src/tui/events/is_up
  - functions/src/tui/events/is_down
  - functions/src/config/secrets/SecretString/len
  - functions/src/tui/events/is_enter
  - functions/src/tui/app/App/open_file_picker
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`async fn handle_file_picker_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [is_up](../../../../../functions/src/tui/events/is_up.md)
- [is_down](../../../../../functions/src/tui/events/is_down.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_enter](../../../../../functions/src/tui/events/is_enter.md)
- [open_file_picker](../../../../../functions/src/tui/app/App/open_file_picker.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)