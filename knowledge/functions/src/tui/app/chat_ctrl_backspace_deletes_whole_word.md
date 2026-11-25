---
type: Rust Function
title: chat_ctrl_backspace_deletes_whole_word
resource: src/tui/app.rs#L4467-L4477
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
  - target: functions/src/tui/app/key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn chat_ctrl_backspace_deletes_whole_word()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)