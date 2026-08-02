---
type: Rust Method
title: handle_approval_key
resource: src/tui/app.rs#L2358-L2399
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/is_approve
  - functions/src/tui/app/App/event_sender
  - functions/src/tui/events/is_deny
  - functions/src/tui/events/is_cancel
  - functions/src/tui/events/is_view_details
  called_by:
  - functions/src/tui/app/App/handle_key_event
---

# Signature

`async fn handle_approval_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_approve](../../../../../functions/src/tui/events/is_approve.md)
- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [is_deny](../../../../../functions/src/tui/events/is_deny.md)
- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [is_view_details](../../../../../functions/src/tui/events/is_view_details.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)