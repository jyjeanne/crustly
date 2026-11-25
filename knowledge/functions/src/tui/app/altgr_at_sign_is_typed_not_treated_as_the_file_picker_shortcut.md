---
type: Rust Function
title: altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut
resource: src/tui/app.rs#L4530-L4543
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
  - target: functions/src/tui/app/key_mod
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)