---
type: Rust Method
title: handle_file_picker_key
resource: src/tui/app.rs#L3286-L3337
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
  - target: functions/src/tui/app/App/open_file_picker
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_file_picker_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [is_up](../../../../../functions/src/tui/events/is_up.md)
- [is_down](../../../../../functions/src/tui/events/is_down.md)
- [is_enter](../../../../../functions/src/tui/events/is_enter.md)
- [open_file_picker](../../../../../functions/src/tui/app/App/open_file_picker.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)