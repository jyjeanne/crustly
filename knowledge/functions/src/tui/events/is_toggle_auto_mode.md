---
type: Rust Function
title: is_toggle_auto_mode
resource: src/tui/events.rs#L515-L517
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/test_toggle_auto_mode_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_toggle_auto_mode(event: &KeyEvent) -> bool`

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [test_toggle_auto_mode_key](../../../../functions/src/tui/events/test_toggle_auto_mode_key.md)