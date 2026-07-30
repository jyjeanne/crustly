---
type: Rust Method
title: handle_sessions_key
resource: src/tui/app.rs#L972-L990
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/is_cancel
  - functions/src/tui/app/App/switch_mode
  - functions/src/tui/events/is_up
  - functions/src/tui/events/is_down
  - functions/src/config/secrets/SecretString/len
  - functions/src/tui/events/is_enter
  - functions/src/tui/app/App/load_session
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`async fn handle_sessions_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [is_up](../../../../../functions/src/tui/events/is_up.md)
- [is_down](../../../../../functions/src/tui/events/is_down.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_enter](../../../../../functions/src/tui/events/is_enter.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)