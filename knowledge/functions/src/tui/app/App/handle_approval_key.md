---
type: Rust Method
title: handle_approval_key
resource: src/tui/app.rs#L3206-L3247
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/is_approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_deny
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_cancel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_view_details
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
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