---
type: Rust Function
title: ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard
resource: src/tui/app.rs#L4598-L4610
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)