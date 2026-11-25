---
type: Rust Function
title: up_moves_the_cursor_inside_a_multiline_draft
resource: src/tui/app.rs#L4323-L4340
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/push_input_history
    resolved_by: tree-sitter
    confidence: exact
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

`async fn up_moves_the_cursor_inside_a_multiline_draft()`

# Calls

- [push_input_history](../../../../functions/src/tui/app/App/push_input_history.md)
- [set_input_text](../../../../functions/src/tui/app/App/set_input_text.md)
- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)