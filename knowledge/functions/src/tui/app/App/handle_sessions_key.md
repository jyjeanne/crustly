---
type: Rust Method
title: handle_sessions_key
resource: src/tui/app.rs#L1641-L1659
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/is_cancel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_down
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_enter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_sessions_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [is_up](../../../../../functions/src/tui/events/is_up.md)
- [is_down](../../../../../functions/src/tui/events/is_down.md)
- [is_enter](../../../../../functions/src/tui/events/is_enter.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)