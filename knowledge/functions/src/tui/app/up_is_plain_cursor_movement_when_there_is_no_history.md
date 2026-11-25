---
type: Rust Function
title: up_is_plain_cursor_movement_when_there_is_no_history
resource: src/tui/app.rs#L4344-L4352
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/set_input_text
    resolved_by: tree-sitter
    confidence: exact
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

`async fn up_is_plain_cursor_movement_when_there_is_no_history()`

# Calls

- [set_input_text](../../../../functions/src/tui/app/App/set_input_text.md)
- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)