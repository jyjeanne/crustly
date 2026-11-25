---
type: Rust Function
title: is_page_up
resource: src/tui/events.rs#L561-L563
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_page_up(event: &KeyEvent) -> bool`

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)