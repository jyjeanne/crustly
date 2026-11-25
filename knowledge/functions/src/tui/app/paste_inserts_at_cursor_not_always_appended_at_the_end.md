---
type: Rust Function
title: paste_inserts_at_cursor_not_always_appended_at_the_end
resource: src/tui/app.rs#L4570-L4583
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn paste_inserts_at_cursor_not_always_appended_at_the_end()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)