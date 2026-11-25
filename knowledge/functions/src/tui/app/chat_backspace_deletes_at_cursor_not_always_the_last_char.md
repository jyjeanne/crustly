---
type: Rust Function
title: chat_backspace_deletes_at_cursor_not_always_the_last_char
resource: src/tui/app.rs#L4420-L4433
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
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn chat_backspace_deletes_at_cursor_not_always_the_last_char()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)