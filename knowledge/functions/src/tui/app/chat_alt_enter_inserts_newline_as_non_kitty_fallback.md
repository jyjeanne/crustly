---
type: Rust Function
title: chat_alt_enter_inserts_newline_as_non_kitty_fallback
resource: src/tui/app.rs#L4394-L4402
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

`async fn chat_alt_enter_inserts_newline_as_non_kitty_fallback()`

# Calls

- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [key](../../../../functions/src/tui/app/key.md)
- [key_mod](../../../../functions/src/tui/app/key_mod.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)